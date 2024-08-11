use uuid::Uuid;

pub struct NewArticle {
    author_id: Uuid,
    title: String,
    text: String,
}

impl NewArticle {
    pub fn new(author_id: Uuid, title: String, text: String) -> Self {
        Self {
            author_id,
            title,
            text,
        }
    }

    pub fn author_id(&self) -> &Uuid {
        &self.author_id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
