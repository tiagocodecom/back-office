use crate::articles::domain::Article;
use async_trait::async_trait;
use uuid::Uuid;

/// A trait for fetching an article by its unique ID.
#[async_trait(?Send)]
pub trait FetchArticlePort {
    /// Retrieves an `Article` by its `UUID`.
    ///
    /// # Parameters
    /// - `article_id`: The unique identifier of the article.
    ///
    /// # Returns
    /// A result containing the `Article` or an error if not found.
    async fn get_article_by_id(&self, article_id: Uuid) -> anyhow::Result<Article>;
}
