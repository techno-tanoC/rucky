#[derive(Debug, Deserialize)]
pub struct Tagging {
    pub name: String,
    pub versions: Vec<String>
}
