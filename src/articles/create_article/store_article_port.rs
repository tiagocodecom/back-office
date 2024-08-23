use crate::articles::create_article::NewArticle;
use async_trait::async_trait;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait(?Send)]
pub trait StoreArticlePort {
    async fn create_article(&self, article: &NewArticle) -> anyhow::Result<Uuid>;
}
