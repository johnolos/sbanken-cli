use core::authorize::Authorize;
use core::entities::Customer;
use reqwest::{Error, Response};
use url::Url;

pub struct CustomersAPI<'a> {
    authorize: &'a Authorize<'a>,
}

impl<'a> CustomersAPI<'a> {
    pub fn new(authorize: &'a Authorize<'a>) -> CustomersAPI {
        CustomersAPI { authorize }
    }

    pub fn get_customer(&self) -> Result<Customer, Error> {
        let url = Url::parse("https://api.sbanken.no/customers/api/v1/customers").unwrap();

        let mut response: Response = self.authorize.get_request(url, None)?;

        return response.json::<Customer>();
    }
}
