use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Article {
    id: Uuid,
    author_id: Uuid,
    title: String,
    content: String,
    created_at: String,
}

pub type ArticlesList = Vec<Article>;

impl Article {
    pub fn new(
        id: Uuid,
        author_id: Uuid,
        title: String,
        content: String,
        created_at: String,
    ) -> Self {
        Self {
            id,
            author_id,
            title,
            content,
            created_at,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn author_id(&self) -> &Uuid {
        &self.author_id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }
}
