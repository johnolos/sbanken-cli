use chrono::{DateTime, SecondsFormat, Utc};
use core::authorize::Authorize;
use core::entities::{Account, Accounts, Transactions, TransferRequest, TransferResponse};
use reqwest::{Error, Response};
use std::collections::HashMap;
use url::Url;

pub struct BankAPI<'a> {
    authorize: &'a Authorize<'a>
}

impl<'a> BankAPI<'a> {
    pub fn new(authorize: &'a Authorize<'a>) -> BankAPI {
        BankAPI { authorize }
    }

    pub fn get_accounts(&self) -> Result<Accounts, Error> {
        let url = Url::parse("https://api.sbanken.no/bank/api/v1/accounts").unwrap();

        let mut response: Response = self.authorize.get_request(url, None)?;

        response.json::<Accounts>()
    }

    pub fn get_account(&self, account: &str) -> Result<Account, Error> {
        let url = Url::parse(&format!(
            "https://api.sbanken.no/bank/api/v1/accounts/{}/",
            account
        )).unwrap();

        let mut response: Response = self.authorize.get_request(url, None)?;

        response.json::<Account>()
    }

    pub fn get_transactions(
        &self,
        account: &str,
        length: i32,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Transactions, Error> {
        let url = Url::parse(&format!(
            "https://api.sbanken.no/bank/api/v1/transactions/{}",
            account
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

        let mut response: Response = self.authorize.get_request(url, Some(params))?;

        response.json::<Transactions>()
    }

    pub fn post_transfer(
        &self,
        transfer: &TransferRequest,
    ) -> Result<TransferResponse, Error> {
        let url = Url::parse("https://api.sbanken.no/bank/api/v1/transfers").unwrap();

        let mut response: Response = self.authorize.post_request(url, transfer)?;

        response.json::<TransferResponse>()
    }
}
