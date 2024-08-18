use crate::articles::domain::NewArticle;
use async_trait::async_trait;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait(?Send)]
pub trait ForCreatingArticle {
    async fn create(&self, article: &NewArticle) -> anyhow::Result<Uuid>;
}
