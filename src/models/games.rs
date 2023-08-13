use super::tags::Tag;

pub struct Game {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub tags: Option<Vec<Tag>>,
}
