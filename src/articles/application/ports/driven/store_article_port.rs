use crate::articles::application::domain::NewArticle;
use async_trait::async_trait;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait(?Send)]
pub trait StoreArticlePort {
    async fn create(&self, article: &NewArticle) -> anyhow::Result<Uuid>;
}
