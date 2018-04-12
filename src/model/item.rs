use model::Group;
use model::User;
use model::Tagging;

#[derive(Debug, Deserialize)]
pub struct Item {
    pub rendered_body: String,
    pub body: String,
    pub coediting: bool,
    pub comments_count: i32,
    pub created_at: String,
    pub group: Option<Group>,
    pub id: String,
    pub likes_count: i32,
    pub private: bool,
    pub reactions_count: i32,
    pub tags: Vec<Tagging>,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: User,
    pub page_views_count: Option<i32>
}
