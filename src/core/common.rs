use core::credentials::Credentials;
use core::entities::AccessToken;
use hyper::header::{Accept, Authorization, Basic, Bearer, ContentType, Headers, UserAgent};
use reqwest::{Client, Response, Error};
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use url::Url;

pub trait Authorize {}

impl Authorize {
    const IDENTITY_SERVER_URL: &'static str = "https://api.sbanken.no/identityserver/connect/token";

    fn get_access_token(credentials: Credentials) -> Result<AccessToken, Error> {
        let mut headers = Headers::new();
        headers.set(UserAgent::new("sbanken-cli/0.2.0"));
        headers.set(Accept::json());
        headers.set(ContentType::form_url_encoded());
        headers.set(Authorization(Basic {
            username: credentials.client_id.to_string(),
            password: Some(credentials.secret.to_string()),
        }));

        let client = Client::builder().default_headers(headers).build().unwrap();
        let mut response = client
            .post(Authorize::IDENTITY_SERVER_URL)
            .body("grant_type=client_credentials")
            .send()?;

        return response.json::<AccessToken>();
    }

    fn construct_headers(token: String) -> Headers {
        let mut headers = Headers::new();

        headers.set(UserAgent::new("sbanken-cli/0.2.0"));
        headers.set(ContentType::json());
        headers.set(Authorization(Bearer { token }));

        return headers;
    }

    pub fn get_request(
        url: Url,
        credentials: Credentials,
        params: Option<HashMap<&str, String>>,
    ) -> Result<Response, Error> {
        let token: AccessToken = Authorize::get_access_token(credentials)?;

        let headers = Authorize::construct_headers(token.access_token);

        let client = Client::builder().default_headers(headers).build()?;

        if let Some(params) = params {
            return client.get(url).query(&params).send();
        }

        return client.get(url).send();
    }

    pub fn post_request(url: Url, credentials: Credentials, object: impl Serialize) -> Result<Response, Error> {
        let token: AccessToken = Authorize::get_access_token(credentials)?;
        let headers = Authorize::construct_headers(token.access_token);

        let client = Client::builder().default_headers(headers).build().unwrap();

        return client
            .post(url)
            .body(serde_json::to_string(&object).unwrap())
            .send();
    }
}
