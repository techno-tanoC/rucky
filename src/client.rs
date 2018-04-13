use std::marker::PhantomData;

use auth::*;
use page::*;
use api::*;

#[derive(Clone, Debug)]
pub struct Client<T> {
    token: Option<String>,
    phantom: PhantomData<T>
}

impl Client<Unauthed> {
    pub fn new() -> Client<Unauthed> {
        Client {
            token: None,
            phantom: PhantomData
        }
    }
}

impl Client<Authed> {
    pub fn auth(token: &str) -> Client<Authed> {
        Client {
            token: Some(token.to_string()),
            phantom: PhantomData
        }
    }
}

impl<T> Client<T> {
    pub fn request(&self) -> API<T> {
        API {
            token: self.token.clone(),
            page: None,
            phantom: PhantomData
        }
    }

    pub fn request_with(&self, page: i32, per_page: i32) -> API<T> {
        API {
            token: self.token.clone(),
            page: Some(Page::new(page, per_page)),
            phantom: PhantomData
        }
    }
}
