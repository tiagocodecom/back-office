use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct NewArticle {
    author_id: Uuid,
    title: String,
    content: String,
}

impl NewArticle {
    pub fn new(author_id: Uuid, title: String, content: String) -> Self {
        Self {
            author_id,
            title,
            content,
        }
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
}
