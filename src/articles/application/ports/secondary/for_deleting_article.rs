use async_trait::async_trait;
use uuid::Uuid;

#[async_trait(?Send)]
pub trait ForDeletingArticle {
    async fn delete(&self, article_id: &Uuid) -> anyhow::Result<Uuid>;
}
