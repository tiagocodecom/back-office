use crate::articles::domain::ArticlesList;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait ForGettingArticles {
    async fn find_articles(&self) -> anyhow::Result<ArticlesList>;
}
