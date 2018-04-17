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
    //Tag
    pub fn tags(&self) -> Option<Vec<Tag>> {
        let target = "https://qiita.com/api/v2/tags";
        let url = self.build_url(target);
        Req::alter_get(&url, &self.token)
    }

    pub fn tag(&self, tag_id: &str) -> Option<Tag> {
        let url = format!("https://qiita.com/api/v2/tags/{}", tag_id);
        Req::alter_get(&url, &self.token)
    }

    pub fn user_following_tags(&self, user_id: &str) -> Option<Vec<Tag>> {
        let url = format!("https://qiita.com/api/v2/users/{}/following_tags", user_id);
        Req::alter_get(&url, &self.token)
    }
}

impl API<Authed> {
    pub fn authenticated_user_items(&self) -> Option<Vec<Item>> {
        let target = "https://qiita.com/api/v2/authenticated_user/items";
        let url = self.build_url(target);
        Req::get(&url, &self.token)
    }

    // Tag
    pub fn follow_tag(&self, tag_id: &str) {
        let url = format!("https://qiita.com/api/v2/tags/{}/following", tag_id);
        Req::put(&url, &self.token)
    }

    pub fn unfollow_tag(&self, tag_id: &str) {
        let url = format!("https://qiita.com/api/v2/tags/{}/following", tag_id);
        Req::delete(&url, &self.token)
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
