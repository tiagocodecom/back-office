use crate::articles::get_article::GetArticleError;
use crate::articles::Article;
use async_trait::async_trait;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

/// A trait for fetching an article by its unique ID.
#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait GetArticlePort {
    /// Retrieves an `Article` by its `UUID`.
    ///
    /// # Parameters
    /// - `article_id`: The unique identifier of the article.
    ///
    /// # Returns
    /// A result containing the `Article` or an error if not found.
    async fn get_article_by_id(&self, article_id: Uuid) -> Result<Article, GetArticleError>;
}
