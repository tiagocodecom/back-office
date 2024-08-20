use crate::articles::application::domain::NewArticle;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait(?Send)]
pub trait CreateArticleUseCase {
    async fn execute(&self, article: &NewArticle) -> anyhow::Result<Uuid>;
}
