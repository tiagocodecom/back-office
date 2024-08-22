use uuid::Uuid;

/// Represents a new article to be created, containing the author's ID, title, and content.
#[derive(Debug, Clone, PartialEq)]
pub struct NewArticle {
    author_id: Uuid,
    title: String,
    content: String,
}

impl NewArticle {
    /// Creates a new `NewArticle` with the specified author ID, title, and content.
    ///
    /// # Parameters
    /// - `author_id`: The UUID of the author.
    /// - `title`: The title of the article.
    /// - `content`: The content of the article.
    pub fn new(author_id: Uuid, title: String, content: String) -> Self {
        Self {
            author_id,
            title,
            content,
        }
    }

    /// Returns a reference to the author's ID.
    pub fn author_id(&self) -> &Uuid {
        &self.author_id
    }

    /// Returns a reference to the article's title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns a reference to the article's content.
    pub fn content(&self) -> &str {
        &self.content
    }
}
