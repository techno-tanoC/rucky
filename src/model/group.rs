#[derive(Debug, Deserialize)]
pub struct Group {
    pub created_at: String,
    pub id: i32,
    pub name: String,
    pub private: bool,
    pub updated_at: String,
    pub url_name: String
}
