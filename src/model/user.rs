#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct User {
    pub description: Option<String>,
    pub facebook_id: Option<String>,
    pub followees_count: i32,
    pub followers_count: i32,
    pub github_login_name: Option<String>,
    pub id: String,
    pub items_count: i32,
    pub linkedin_id: Option<String>,
    pub location: Option<String>,
    pub name: Option<String>,
    pub organization: Option<String>,
    pub permanent_id: i32,
    pub profile_image_url: String,
    pub twitter_screen_name: Option<String>,
    pub website_url: Option<String>
}
