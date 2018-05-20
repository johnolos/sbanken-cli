use core::common::Authorize;
use core::credentials::Credentials;
use core::entities::Customer;
use reqwest::{Error, Response};
use url::Url;

pub struct CustomersAPI {}

impl CustomersAPI {
    pub fn get_customer(credentials: &Credentials) -> Result<Customer, Error> {
        let url = Url::parse(&format!(
            "https://api.sbanken.no/customers/api/v1/customers/{}",
            credentials.customer_id
        )).unwrap();

        let mut response: Response = Authorize::get_request(url, credentials, None)?;

        return response.json::<Customer>();
    }
}
