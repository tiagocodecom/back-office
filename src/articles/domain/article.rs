use crate::articles::domain::Author;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Article {
    id: Uuid,
    author: Author,
    title: String,
    text: String,
    created_at: String,
}

pub type ArticlesList = Vec<Article>;

impl Article {
    pub fn new(id: Uuid, author: Author, title: String, text: String, created_at: String) -> Self {
        Self {
            id,
            author,
            title,
            text,
            created_at,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn author_id(&self) -> &Uuid {
        &self.author.id()
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }
}
