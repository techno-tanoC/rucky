use std::marker::PhantomData;

use auth::*;
use page::*;
use req::*;
use model::*;

pub struct API<T> {
    pub(crate) token: Option<String>,
    pub(crate) page: Option<Page>,
    pub(crate) phantom: PhantomData<T>
}

impl<T> API<T> {
    pub fn get_tags(&self) -> Option<Vec<Tag>> {
        let target = "https://qiita.com/api/v2/tags";
        let url = self.build_url(target);
        Req::get(&url, &self.token)
    }
}

impl API<Authed> {
    pub fn authenticated_user_items(&self) -> Option<Vec<Item>> {
        let target = "https://qiita.com/api/v2/authenticated_user/items";
        let url = self.build_url(target);
        Req::auth_get(&url, &self.token)
    }
}

impl<T> API<T> {
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
