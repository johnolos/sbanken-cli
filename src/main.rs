extern crate chrono;
extern crate clap;
extern crate hyper;
extern crate regex;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate termion;
#[macro_use]
extern crate text_io;
extern crate time;
extern crate url;

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use clap::{App, Shell};
use core::authorize::Authorize;
use core::bank::BankAPI;
use core::credentials::Credentials;
use core::customers::CustomersAPI;
use core::entities::{AccountObj, Accounts, Transactions, TransferRequest};
use core::error::Error;
use core::interactive::{fuzzy_match_account, remove_account};
use std::env;
use std::io;
use termion::{color, style};
use time::Duration;

mod cli;
mod core;

fn main() -> Result<(), Error> {
    let app: App = cli::build_cli();

    let matches = app.get_matches();

    if matches.is_present("generate-bash-completions") {
        cli::build_cli().gen_completions_to("sbanken-cli", Shell::Bash, &mut std::io::stdout());
    }

    if matches.is_present("generate-zsh-completions") {
        cli::build_cli().gen_completions_to("sbanken-cli", Shell::Zsh, &mut std::io::stdout());
    }

    if matches.is_present("generate-fish-completions") {
        cli::build_cli().gen_completions_to("sbanken-cli", Shell::Fish, &mut std::io::stdout());
    }

    if matches.is_present("generate-powershell-completions") {
        cli::build_cli().gen_completions_to(
            "sbanken-cli",
            Shell::PowerShell,
            &mut std::io::stdout(),
        );
    }

    let color_env: bool = match env::var("SBANKEN_COLOR") {
        Ok(value) => match value.as_ref() {
            "1" => true,
            _ => false,
        },
        Err(_) => false,
    };

    let color = color_env || matches.is_present("color");

    let secret: String = match env::var("SBANKEN_SECRET") {
        Ok(secret) => secret,
        Err(_) => {
            return Err(Error::EnvMissing("SBANKEN_SECRET"));
        }
    };

    let client_id: String = match env::var("SBANKEN_CLIENT_ID") {
        Ok(client_id) => client_id,
        Err(_) => {
            return Err(Error::EnvMissing("SBANKEN_CLIENT_ID"));
        }
    };

    let customer_id: String = match env::var("SBANKEN_CUSTOMER_ID") {
        Ok(customer_id) => customer_id,
        Err(_) => {
            return Err(Error::EnvMissing("SBANKEN_CUSTOMER_ID"));
        }
    };

    let credentials = &Credentials::new(secret, client_id, customer_id);

    let authorize = &Authorize::new(credentials);

    let bank_api = &BankAPI::new(authorize);

    let customer_api = &CustomersAPI::new(authorize);

    if let Some(matches) = matches.subcommand_matches("account") {
        if let Some(account_number) = matches.value_of("account") {
            let account = match bank_api.get_account(account_number) {
                Ok(response) => response,
                Err(err) => return Err(Error::Reqwest(err))
            };

            println!("{:}", account);
        } else {
            let response: Accounts = match bank_api.get_accounts() {
                Ok(response) => response,
                Err(err) => return Err(Error::Reqwest(err))
            };

            if matches.is_present("interactive") {
                let account = fuzzy_match_account(&response.items, "Select account")?;

                println!("{:}", account);
            }
            if matches.is_present("list") {
                for account in response.items {
                    println!("{}\t\t[nr: {}]", account.name, account.account_number);
                }
            } else {
                println!("{:}", response);
            }
        }
    }

    if let Some(_matches) = matches.subcommand_matches("customer") {
        let customer = match customer_api.get_customer() {
            Ok(customer) => customer,
            Err(err) => return Err(Error::Reqwest(err)),
        };

        println!("{:}", customer);
    }

    if let Some(matches) = matches.subcommand_matches("transaction") {
        let length: i32 = match matches.value_of("length") {
            Some(length) => match length.parse::<i32>() {
                Ok(integer) => integer,
                Err(_) => return Err(Error::Parsable("given value for length couldn't be parsed to integer"))
            },
            None => 20,
        };

        let end_date: DateTime<Utc> = match matches.value_of("to") {
            Some(end_date) => Utc.from_utc_date(&NaiveDate::parse_from_str(end_date, "%Y-%m-%d")?).and_hms(23, 59, 59),
            None => Utc::now(),
        };

        let start_date: DateTime<Utc> = match matches.value_of("from") {
            Some(to_date) => Utc.from_utc_date(&NaiveDate::parse_from_str(to_date, "%Y-%m-%d")?).and_hms(0, 0, 0),
            None => end_date - Duration::days(30),
        };

        if end_date < start_date {
            return Err(Error::Message("end_date was earlier than start date"));
        }

        let account: String;

        if matches.is_present("interactive") {
            let mut response: Accounts = bank_api.get_accounts()?;

            let accounts: &mut Vec<AccountObj> = &mut response.items;

            account = fuzzy_match_account(&accounts, "Select from_account")?
                .account_id.to_string();

        } else {
            account = match matches.value_of("account") {
                Some(account) => account.to_string(),
                None => {
                    return Err(Error::Parsable("account wasn't parsable"));
                }
            };
        }

        let transactions: Transactions = bank_api.get_transactions(&account, length, start_date, end_date)?;

        println!("{:}", transactions);
    }

    if let Some(matches) = matches.subcommand_matches("transfer") {
        let from_account_id: String;
        let to_account_id: String;
        let amount: f32;
        let message: String;

        if matches.is_present("interactive") {

            let mut response: Accounts = bank_api.get_accounts()?;

            let accounts: &mut Vec<AccountObj> = &mut response.items;

            from_account_id = fuzzy_match_account(&accounts, "Select from_account")?
                .account_id.to_string();

            remove_account(accounts, &from_account_id);

            to_account_id = fuzzy_match_account(&accounts, "Select to_account")?
                .account_id.to_string();

            println!("Amount: ");
            amount = read!("{}\n");

            println!("Message: ");
            message = read!("{}\n");
        } else {
            from_account_id = match matches.value_of("from") {
                Some(value) => value.to_string(),
                None => return Err(Error::ArgumentMissing("from"))
            };

            to_account_id = match matches.value_of("to") {
                Some(value) => value.to_string(),
                None => return Err(Error::ArgumentMissing("to"))
            };

            amount = match matches.value_of("amount") {
                Some(amount) => match amount.parse::<f32>() {
                    Ok(amount) => amount,
                    Err(_) => {
                        return Err(Error::Parsable("amount couldn't be parsed to a float value"));
                    }
                },
                None => return Err(Error::ArgumentMissing("amount")),
            };

            message = match matches.value_of("message") {
                Some(value) => value.to_string(),
                None => return Err(Error::ArgumentMissing("message"))
            };
        }

        let transfer = TransferRequest {
            from_account_id,
            to_account_id,
            message,
            amount,
        };

        let transfer = bank_api.post_transfer(&transfer)?;

        if transfer.is_error {
            return Err(Error::Message("couldn't perform your transfer"));
        }

        if color {
            println!(
                "{}Your transfer was successfully executed.{}",
                color::Fg(color::Green),
                style::Reset
            )
        } else {
            println!("Your transfer was successfully executed.")
        }
    }

    Ok(())
}
