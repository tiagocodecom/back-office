use crate::articles::entities::{Error, NewArticle};
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait StoreArticleService {
    type Output;

    async fn execute(&self, new_article: &NewArticle) -> Result<Self::Output, Error>;
}
