use crate::articles::domain::Article;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait(?Send)]
pub trait ForGettingArticle {
    async fn get_by_id(&self, article_id: &Uuid) -> anyhow::Result<Article>;
}
