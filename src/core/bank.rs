use chrono::{DateTime, SecondsFormat, Utc};
use core::common::Authorize;
use core::credentials::Credentials;
use core::entities::{Account, Accounts, Transactions, TransferRequest, TransferResponse};
use reqwest::{Error, Response};
use std::collections::HashMap;
use url::Url;

pub struct BankAPI {}

impl BankAPI {
    pub fn get_accounts(credentials: &Credentials) -> Result<Accounts, Error> {
        let url = Url::parse(&format!(
            "https://api.sbanken.no/bank/api/v1/accounts/{}",
            credentials.customer_id
        )).unwrap();

        let mut response: Response = Authorize::get_request(url, credentials, None)?;

        return response.json::<Accounts>();
    }

    pub fn get_account(credentials: &Credentials, account: &str) -> Result<Account, Error> {
        let url = Url::parse(&format!(
            "https://api.sbanken.no/bank/api/v1/accounts/{}/{}",
            credentials.customer_id, account
        )).unwrap();

        let mut response: Response = Authorize::get_request(url, credentials, None)?;

        return response.json::<Account>();
    }

    pub fn get_transactions(
        credentials: &Credentials,
        account: &str,
        length: i32,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Transactions, Error> {
        let url = Url::parse(&format!(
            "https://api.sbanken.no/bank/api/v1/transactions/{}/{}",
            credentials.customer_id, account
        )).unwrap();

        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("length", length.to_string());
        params.insert(
            "startDate",
            start_date.to_rfc3339_opts(SecondsFormat::Secs, true),
        );
        params.insert(
            "endDate",
            end_date.to_rfc3339_opts(SecondsFormat::Secs, true),
        );

        let mut response: Response = Authorize::get_request(url, credentials, Some(params))?;

        return response.json::<Transactions>();
    }

    pub fn post_transfer(
        credentials: &Credentials,
        transfer: TransferRequest,
    ) -> Result<TransferResponse, Error> {
        let url = Url::parse(&format!(
            "https://api.sbanken.no/bank/api/v1/transfers/{}",
            credentials.customer_id,
        )).unwrap();

        let mut response: Response = Authorize::post_request(url, credentials, &transfer)?;

        return response.json::<TransferResponse>();
    }
}
