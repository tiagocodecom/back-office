use crate::articles::application::ports::ForGettingArticles;
use crate::articles::application::ports::ForSavingArticle;
use crate::articles::domain::{Article, ArticlesList, NewArticle};
use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct ArticleRepository {}

#[async_trait(?Send)]
impl ForGettingArticles for ArticleRepository {
    async fn find_articles(&self) -> anyhow::Result<ArticlesList> {
        todo!()
    }
}

#[async_trait(?Send)]
impl ForSavingArticle for ArticleRepository {
    async fn save_article(
        &self,
        db_pool: &PgPool,
        article: &NewArticle,
    ) -> anyhow::Result<Article> {
        todo!()
    }
}
