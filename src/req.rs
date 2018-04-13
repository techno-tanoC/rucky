use reqwest::{self, Client, header::*};
use serde::de::DeserializeOwned;

pub struct Req;

impl Req {
    pub fn alter_get<D: DeserializeOwned>(url: &str, token: &Option<String>) -> Option<D> {
        if token.is_some() {
            Self::get(url, token)
        } else {
            reqwest::get(url)
                .and_then(|mut res| res.json())
                .ok()
        }
    }

    pub fn get<D: DeserializeOwned>(url: &str, token: &Option<String>) -> Option<D> {
        let auth = Self::build_bearer(token);
        Client::new()
            .get(url)
            .header(Authorization(auth))
            .send()
            .and_then(|mut res| res.json())
            .ok()
    }

    pub fn put(url: &str, token: &Option<String>) {
        let bearer = Self::build_bearer(token);
        let _ = Client::new()
            .put(url)
            .header(Authorization(bearer))
            .send();
    }

    pub fn delete(url: &str, token: &Option<String>) {
        let auth = Self::build_bearer(token);
        let _ =Client::new()
            .delete(url)
            .header(Authorization(auth))
            .send();
    }

    fn build_bearer(token: &Option<String>) -> Bearer {
        Bearer {
            token: token
                .clone()
                .expect("the token must not be empty")
                .to_string()
        }
    }
}
