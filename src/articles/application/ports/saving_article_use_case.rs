use crate::articles::domain::NewArticle;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait(?Send)]
pub trait SaveArticleUseCase {
    async fn save_article(&self, article: &NewArticle) -> anyhow::Result<Uuid>;
}
