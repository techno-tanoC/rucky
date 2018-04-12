use std::marker::PhantomData;

use auth::*;
use page::*;
use req::*;

#[derive(Clone, Debug)]
pub struct Client<T> {
    token: String,
    phantom: PhantomData<T>
}

impl Client<Unauthed> {
    pub fn new() -> Client<Unauthed> {
        Client {
            token: "".to_string(),
            phantom: PhantomData
        }
    }
}

impl Client<Authed> {
    pub fn auth(token: &str) -> Client<Authed> {
        Client {
            token: token.to_string(),
            phantom: PhantomData
        }
    }
}

impl<T> Client<T> {
    pub fn request(&self) -> Req<T> {
        Req {
            token: self.token.clone(),
            page: None,
            phantom: PhantomData
        }
    }

    pub fn request_with(&self, page: i32, per_page: i32) -> Req<T> {
        Req {
            token: self.token.clone(),
            page: Some(Page::new(page, per_page)),
            phantom: PhantomData
        }
    }
}
