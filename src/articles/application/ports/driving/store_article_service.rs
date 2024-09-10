use crate::articles::entities::{ArticleError, NewArticle};
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait StoreArticleService {
    type Output;

    async fn execute(&self, new_article: &NewArticle) -> Result<Self::Output, ArticleError>;
}
