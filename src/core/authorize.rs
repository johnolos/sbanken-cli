use core::credentials::Credentials;
use core::entities::AccessToken;
use hyper::header::{Accept, Authorization, Basic, Bearer, ContentType, Headers, UserAgent};
use reqwest::{Client, Error, Response};
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use url::form_urlencoded::byte_serialize;
use url::Url;

pub struct Authorize<'a> {
    pub credentials: &'a Credentials,
}

impl<'a> Authorize<'a> {
    const IDENTITY_SERVER_URL: &'static str = "https://api.sbanken.no/identityserver/connect/token";

    pub fn new(credentials: &'a Credentials) -> Authorize {
        Authorize { credentials }
    }

    fn get_access_token(&self) -> Result<AccessToken, Error> {
        let username: String = byte_serialize(self.credentials.client_id.as_bytes()).collect();
        let password: Option<String> = Some(byte_serialize(self.credentials.secret.as_bytes()).collect());

        let mut headers = Headers::new();
        headers.set(UserAgent::new("sbanken-cli/0.3.0"));
        headers.set(Accept::json());
        headers.set(ContentType::form_url_encoded());
        headers.set(Authorization(Basic {
            username,
            password
        }));

        let client = Client::builder().default_headers(headers).build()?;
        let mut response = client
            .post(Authorize::IDENTITY_SERVER_URL)
            .body("grant_type=client_credentials")
            .send()?;

        response.json::<AccessToken>()
    }

    fn construct_headers(&self, token: String) -> Headers {
        let mut headers = Headers::new();

        headers.set(UserAgent::new("sbanken-cli/0.3.0"));
        headers.set(ContentType::json());
        headers.set(Authorization(Bearer { token }));
        headers.set_raw("customerId", self.credentials.customer_id.to_string());

        headers
    }

    pub fn get_request(
        &self,
        url: Url,
        params: Option<HashMap<&str, String>>,
    ) -> Result<Response, Error> {
        let token: AccessToken = self.get_access_token()?;

        let headers: Headers = self.construct_headers(token.access_token);

        let client = Client::builder().default_headers(headers).build()?;

        if let Some(params) = params {
            return client.get(url).query(&params).send();
        }

        client.get(url).send()
    }

    pub fn post_request(
        &self,
        url: Url,
        object: impl Serialize,
    ) -> Result<Response, Error> {
        let token: AccessToken = self.get_access_token()?;
        let headers = self.construct_headers(token.access_token);

        let client = Client::builder().default_headers(headers).build()?;

        client
            .post(url)
            .body(serde_json::to_string(&object).unwrap())
            .send()
    }
}
