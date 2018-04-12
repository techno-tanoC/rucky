use user::User;

#[derive(Debug, Deserialize)]
pub struct Comment {
    pub body: String,
    pub created_at: String,
    pub id: String,
    pub rendered_body: String,
    pub updated_at: String,
    pub user: User
}
