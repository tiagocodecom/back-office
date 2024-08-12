use crate::articles::domain::{Article, NewArticle};
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait SaveArticleUseCase {
    async fn save_article(&self, article: &NewArticle) -> anyhow::Result<Article>;
}
