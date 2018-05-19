use core::credentials::Credentials;
use core::entities::AccessToken;
use hyper::header::{Accept, Authorization, Basic, Bearer, ContentType, Headers, UserAgent};
use reqwest::{Client, Response};
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use std::process;
use termion::{color, style};
use url::Url;

pub trait Authorize {}

impl Authorize {
    const IDENTITY_SERVER_URL: &'static str = "https://api.sbanken.no/identityserver/connect/token";

    fn get_access_token(credentials: Credentials) -> AccessToken {
        let mut headers = Headers::new();
        headers.set(UserAgent::new("sbanken-cli/0.1.0"));
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
            .send()
            .unwrap();

        if response.status().is_success() {
            return response.json::<AccessToken>().unwrap();
        } else if response.status().is_server_error() {
            eprintln!(
                "{}Unable to contact authentication server{}",
                color::Fg(color::Red),
                style::Reset
            );

            process::exit(0x1011);
        }

        eprintln!("{}Unknown error{}", color::Fg(color::Red), style::Reset);
        process::exit(0x1100);
    }

    fn construct_headers(token: String) -> Headers {
        let mut headers = Headers::new();

        headers.set(UserAgent::new("sbanken-cli/0.1.0"));
        headers.set(ContentType::json());
        headers.set(Authorization(Bearer { token }));

        return headers;
    }

    pub fn get_request(
        url: Url,
        credentials: Credentials,
        params: Option<HashMap<&str, String>>,
    ) -> Response {
        let token: AccessToken = Authorize::get_access_token(credentials);

        let headers = Authorize::construct_headers(token.access_token);

        let client = Client::builder().default_headers(headers).build().unwrap();

        if let Some(params) = params {
            return client.get(url).query(&params).send().unwrap();
        }

        return client.get(url).send().unwrap();
    }

    pub fn post_request(url: Url, credentials: Credentials, object: impl Serialize) -> Response {
        let token: AccessToken = Authorize::get_access_token(credentials);
        let headers = Authorize::construct_headers(token.access_token);

        let client = Client::builder().default_headers(headers).build().unwrap();

        return client
            .post(url)
            .body(serde_json::to_string(&object).unwrap())
            .send()
            .unwrap();
    }
}
