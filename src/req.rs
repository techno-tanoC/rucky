use reqwest;
use reqwest::header::*;
use serde::de::DeserializeOwned;

use page::*;

pub struct Req;

impl Req {
    pub fn get<D: DeserializeOwned>(url: &str, token: &Option<String>) -> Option<D> {
        reqwest::get(url)
            .and_then(|mut res| res.json())
            .ok()
    }

    pub fn auth_get<D: DeserializeOwned>(url: &str, token: &Option<String>) -> Option<D> {
        let access_token = Bearer {
            token: token
                .clone()
                .expect("the token must not be empty")
                .to_string()
        };

        reqwest::Client::new()
            .get(url)
            .header(Authorization(access_token))
            .send()
            .and_then(|mut res| res.json())
            .ok()
    }
}
