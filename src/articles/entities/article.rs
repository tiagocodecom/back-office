use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter, Result};
use uuid::Uuid;

/// A list of articles, represented as a vector of `Article` structs.
pub type ArticlesList = Vec<Article>;

/// Represents an article with metadata such as ID, author ID, title, content, and creation timestamp.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Article {
    id: Uuid,
    author_id: Uuid,
    title: String,
    content: String,
    created_at: String,
}

impl Article {
    /// Creates a new `Article` with the specified parameters.
    ///
    /// # Parameters
    /// - `id`: The unique identifier for the article.
    /// - `author_id`: The unique identifier of the author.
    /// - `title`: The title of the article.
    /// - `content`: The content of the article.
    /// - `created_at`: The creation timestamp of the article.
    ///
    /// # Returns
    /// A new instance of `Article`.
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

    /// Returns a reference to the article's unique identifier.
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Returns a reference to the author's unique identifier.
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

    /// Returns a reference to the article's creation timestamp.
    pub fn created_at(&self) -> &str {
        &self.created_at
    }
}

impl Debug for Article {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Article {{ id: {}, author_id: {}, title: {}, content: {}, created_at: {} }}",
            self.id.to_string(),
            self.author_id.to_string(),
            self.title,
            self.content,
            self.created_at
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_an_article_and_returns_the_correct_properties() {
        let id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        let title = "Hello world".to_string();
        let content = "This is the content of the article.".to_string();
        let created_at = "2021-08-01T12:00:00Z".to_string();
        let article = Article::new(
            id,
            author_id,
            title.clone(),
            content.clone(),
            created_at.clone(),
        );

        assert_eq!(article.id(), &id);
        assert_eq!(article.author_id(), &author_id);
        assert_eq!(article.title(), title);
        assert_eq!(article.content(), content);
        assert_eq!(article.created_at(), created_at);
    }
}
