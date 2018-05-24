extern crate clap;
extern crate hyper;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate termion;
extern crate time;
extern crate url;

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use clap::{App, Shell};
use core::bank::BankAPI;
use core::credentials::Credentials;
use core::authorize::Authorize;
use core::customers::CustomersAPI;
use core::entities::{Accounts, Transactions, TransferRequest};
use core::error::CliError;
use std::env;
use termion::{color, style};
use time::Duration;

mod cli;
mod core;

fn main() -> Result<(), CliError> {
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
            return Err(CliError::new("env SBANKEN_SECRET was missing", color));
        }
    };

    let client_id: String = match env::var("SBANKEN_CLIENT_ID") {
        Ok(client_id) => client_id,
        Err(_) => {
            return Err(CliError::new("env SBANKEN_CLIENT_ID was missing{}", color));
        }
    };

    let customer_id: String = match env::var("SBANKEN_CUSTOMER_ID") {
        Ok(customer_id) => customer_id,
        Err(_) => {
            return Err(CliError::new("env SBANKEN_CUSTOMER_ID was missing", color));
        }
    };

    let ref credentials = Credentials::new(secret, client_id, customer_id);

    let ref authorize = Authorize::new(credentials);

    let ref bank_api = BankAPI::new(authorize);

    let ref customer_api = CustomersAPI::new(authorize);

    if let Some(matches) = matches.subcommand_matches("account") {
        if let Some(account_number) = matches.value_of("account") {
            let account = match bank_api.get_account(account_number) {
                Ok(response) => response,
                Err(_) => {
                    return Err(CliError::new(
                        "an error occurred while trying to retrieve account information",
                        color,
                    ))
                }
            };

            println!("{:}", account);
        } else {
            let response: Accounts = match bank_api.get_accounts() {
                Ok(response) => response,
                Err(_) => {
                    return Err(CliError::new(
                        "an error occurred while trying to retrieve account information",
                        color,
                    ))
                }
            };

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
            Err(_) => return Err(CliError::new("unable to contact the customer api", color)),
        };

        println!("{:}", customer);
    }

    if let Some(matches) = matches.subcommand_matches("transaction") {
        let account = match matches.value_of("account") {
            Some(account) => account,
            None => {
                return Err(CliError::new("account wasn't parsable{}", color));
            }
        };

        let length: i32 = match matches.value_of("length") {
            Some(length) => match length.parse::<i32>() {
                Ok(integer) => integer,
                Err(_) => {
                    return Err(CliError::new(
                        "given value for length couldn't be parsed to integer",
                        color,
                    ));
                }
            },
            None => 20,
        };

        let end_date: DateTime<Utc> = match matches.value_of("to") {
            Some(end_date) => match NaiveDate::parse_from_str(end_date, "%Y-%m-%d") {
                Ok(valid_date) => Utc.from_utc_date(&valid_date).and_hms(23, 59, 59),
                Err(_) => {
                    return Err(CliError::new(
                        "given value for end_date couldn't be parsed to a UTC date",
                        color,
                    ));
                }
            },
            None => Utc::now(),
        };

        let start_date: DateTime<Utc> = match matches.value_of("from") {
            Some(to_date) => match NaiveDate::parse_from_str(to_date, "%Y-%m-%d") {
                Ok(valid_date) => Utc.from_utc_date(&valid_date).and_hms(0, 0, 0),
                Err(_) => {
                    return Err(CliError::new(
                        "given value for end_date couldn't be parsed to a UTC date",
                        color,
                    ));
                }
            },
            None => end_date - Duration::days(30),
        };

        if end_date < start_date {
            return Err(CliError::new("end_date was earlier than start date", color));
        }

        let transactions: Transactions =
            match bank_api.get_transactions(account, length, start_date, end_date) {
                Ok(success) => success,
                Err(_) => return Err(CliError::new("couldn't retrieve transactions", color)),
            };

        println!("{:}", transactions);
    }

    if let Some(matches) = matches.subcommand_matches("transfer") {
        let from_account_id: String = match matches.value_of("from") {
            Some(account) => account.to_string(),
            None => return Err(CliError::new("missing from argument", color)),
        };

        let to_account_id: String = match matches.value_of("to") {
            Some(account) => account.to_string(),
            None => return Err(CliError::new("missing to argument", color)),
        };

        let amount: f32 = match matches.value_of("amount") {
            Some(amount) => match amount.parse::<f32>() {
                Ok(amount) => amount,
                Err(_) => {
                    return Err(CliError::new(
                        "given value for amount couldn't be parsed to a float value",
                        color,
                    ));
                }
            },
            None => return Err(CliError::new("missing amount argument", color)),
        };

        let message: String = match matches.value_of("message") {
            Some(message) => message.to_string(),
            None => return Err(CliError::new("missing message argument", color)),
        };

        let transfer = TransferRequest {
            from_account_id,
            to_account_id,
            message,
            amount,
        };

        let transfer = match bank_api.post_transfer(transfer) {
            Ok(transfer) => transfer,
            Err(_) => return Err(CliError::new("couldn't transfer money", color)),
        };

        if transfer.is_error {
            match transfer.error_message {
                Some(msg) => {
                    return Err(CliError::new(&msg, color));
                }
                None => {
                    return Err(CliError::new("couldn't perform your transfer", color));
                }
            }
        } else {
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
    }

    Ok(())
}
