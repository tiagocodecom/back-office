use crate::articles::entities::{Error, NewArticle};
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

/// A trait for storing a new article in the persistence system.
#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait StoreArticleRepository {
    /// Stores a `NewArticle`.
    ///
    /// # Parameters
    /// - `new_article`: The basic data required for creating an `Article`.
    ///
    /// # Returns
    /// A result containing the `Article` or an error if something failed.
    async fn store_article(&self, new_article: &NewArticle) -> Result<Uuid, Error>;
}
