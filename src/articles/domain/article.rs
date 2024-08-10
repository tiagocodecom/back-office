use crate::articles::domain::author::Author;

#[derive(Debug, Clone)]
pub struct Article {
    id: uuid::Uuid,
    author: Author,
    title: String,
    text: String,
    created_at: String,
}

pub type ArticlesList = Vec<Article>;

impl Article {
    pub fn new(
        id: uuid::Uuid,
        author: Author,
        title: String,
        text: String,
        created_at: String,
    ) -> Self {
        Self {
            id,
            author,
            title,
            text,
            created_at,
        }
    }
}
