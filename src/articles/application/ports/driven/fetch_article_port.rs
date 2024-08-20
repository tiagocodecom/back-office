use crate::articles::application::domain::Article;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait(?Send)]
pub trait FetchArticlePort {
    async fn get_by_id(&self, article_id: &Uuid) -> anyhow::Result<Article>;
}
