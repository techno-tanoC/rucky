use std::borrow::Cow;
use std::marker::PhantomData;

use reqwest;
use reqwest::header::*;
use serde::de::DeserializeOwned;

use auth::*;
use page::*;
use model::*;

pub struct Req<T> {
    pub(crate) token: String,
    pub(crate) page: Option<Page>,
    pub(crate) phantom: PhantomData<T>
}

impl<T> Req<T> {
    pub fn get_tags(&self) -> Option<Vec<Tag>> {
        let url = "https://qiita.com/api/v2/tags";
        Self::get(url)
    }
}

impl Req<Authed> {
    pub fn authenticated_user_items(&self) -> Option<Vec<Item>> {
        let target = "https://qiita.com/api/v2/authenticated_user/items";
        let url = self.build_url(target);
        Self::auth_get(&self, &url)
    }
}

impl<T> Req<T> {
    fn get<D: DeserializeOwned>(url: &str) -> Option<D> {
        reqwest::get(url).and_then(|mut res| res.json()).ok()
    }

    fn auth_get<D: DeserializeOwned>(&self, url: &str) -> Option<D> {
        let access_token = Bearer {
            token: self.token.clone()
        };

        reqwest::Client::new()
            .get(url)
            .header(Authorization(access_token))
            .send()
            .and_then(|mut res| res.json())
            .ok()
    }

    fn build_url(&self, url: &str) -> String {
        match self.page {
            Some(ref p) => {
                p.build_url(url)
            },
            None =>
                url.to_string()
        }
    }
}
