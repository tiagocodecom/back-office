use crate::articles::domain::Article;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait ForUpdatingArticle {
    async fn update(&self, article: &Article) -> anyhow::Result<Article>;
}
