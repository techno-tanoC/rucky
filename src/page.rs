pub struct Page {
    page: i32,
    per_page: i32
}

impl Page {
    pub fn new(page: i32, per_page: i32) -> Page {
        Page {
            page: page,
            per_page: per_page
        }
    }

    pub fn build_url(&self, url: &str) -> String {
        let query = self.build_query();
        format!("{}{}", url, query)
    }
 
    pub fn build_query(&self) -> String {
        format!("?page={}&per_page={}", &self.page, &self.per_page)
    }
}
