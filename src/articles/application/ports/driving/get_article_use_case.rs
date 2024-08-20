use crate::articles::application::domain::Article;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait(?Send)]
pub trait GetArticleUseCase {
    async fn execute(&self, article_id: &Uuid) -> anyhow::Result<Article>;
}
