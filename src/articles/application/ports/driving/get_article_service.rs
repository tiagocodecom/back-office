use crate::articles::entities::ArticleError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait(?Send)]
pub trait GetArticleService {
    type Output;

    async fn execute(&self, article_id: &Uuid) -> Result<Self::Output, ArticleError>;
}
