use crate::articles::entities::{Slug, Status};
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use derive_getters::Getters;
use std::fmt::{Debug, Formatter, Result};
use uuid::Uuid;

/// A list of articles, represented as a vector of `Article` structs.
pub type ArticlesList = Vec<Article>;

/// Represents an article with metadata such as ID, author ID, title, content, and creation timestamp.
#[derive(PartialEq, Clone, Builder, Getters)]
pub struct Article {
    id: Uuid,
    likes: i32,
    title: String,
    slug: Slug,
    summary: String,
    content: String,
    author_id: Uuid,
    thumbnail_uri: String,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl Debug for Article {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Article {{ id: {}, author_id: {}, title: {}, summary: {}, created_at: {} }}",
            self.id().to_string(),
            self.author_id().to_string(),
            self.title(),
            self.summary(),
            self.created_at()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::assert_none;

    #[test]
    fn creates_an_article_and_returns_the_correct_properties() {
        let id = Uuid::new_v4();
        let likes = 5;
        let title = "Hello world".to_string();
        let slug = Slug::new("hello-world".into());
        let summary = "This is the summary of the article.".to_string();
        let content = "This is the content of the article.".to_string();
        let thumbnail_uri = "https://example.com/thumbnail.jpg".to_string();
        let author_id = Uuid::new_v4();
        let status = Status::Draft;
        let created_at = Utc::now();
        let article = Article {
            id: id.clone(),
            likes,
            title: title.clone(),
            slug: slug.clone(),
            summary: summary.clone(),
            content: content.clone(),
            author_id: author_id.clone(),
            thumbnail_uri: thumbnail_uri.clone(),
            status: status.clone(),
            created_at: created_at.clone(),
            updated_at: None,
        };

        assert_eq!(article.id(), &id);
        assert_eq!(article.author_id(), &author_id);
        assert_eq!(article.title(), title.as_str());
        assert_eq!(article.slug().as_ref(), slug.as_ref());
        assert_eq!(article.summary(), summary.as_str());
        assert_eq!(article.content(), content.as_str());
        assert_eq!(article.likes().clone(), 5);
        assert_eq!(article.created_at().timestamp(), created_at.timestamp());
        assert_eq!(article.status().as_ref(), status.as_ref());
        assert_none!(article.updated_at());
    }
}
