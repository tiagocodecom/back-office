use crate::articles::domain::{Article, NewArticle};
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait(?Send)]
pub trait ForSavingArticle {
    async fn save_article(&self, db_pool: &PgPool, article: &NewArticle)
        -> anyhow::Result<Article>;
}
