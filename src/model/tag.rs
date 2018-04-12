#[derive(Debug, Deserialize)]
pub struct Tag {
    pub followers_count: i32,
    pub icon_url: Option<String>,
    pub id: String,
    pub items_count: i32
}
