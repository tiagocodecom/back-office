use crate::articles::create_article::{NewArticle, StoreArticlePort};
use crate::articles::get_article::GetArticleError;
use crate::articles::get_article::GetArticlePort;
use crate::articles::Article;
use anyhow::Context;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::{query, Row};
use sqlx::{Executor, PgPool};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PostgresArticleRepository {
    pool: Arc<PgPool>,
}

impl PostgresArticleRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait(?Send)]
impl StoreArticlePort for &PostgresArticleRepository {
    #[tracing::instrument(name = "Store article in the database", skip(self, article))]
    async fn create_article(&self, article: &NewArticle) -> anyhow::Result<Uuid> {
        let article_id = Uuid::new_v4();
        let created_at = Utc::now();

        let query = query!(
            "INSERT INTO articles (id, author_id, title, content, created_at) VALUES ($1, $2, $3, $4, $5)",
            &article_id,
            &article.author_id(),
            &article.title(),
            &article.content(),
            &created_at,
        );

        self.pool
            .execute(query)
            .await
            .context("Failed to save the article")?;

        Ok(article_id)
    }
}

#[async_trait(?Send)]
impl GetArticlePort for &PostgresArticleRepository {
    #[tracing::instrument(name = "Fetch article from the database", skip(self, article_id))]
    async fn get_article_by_id(&self, article_id: Uuid) -> Result<Article, GetArticleError> {
        let query = query!(
            "SELECT id, author_id, title, content, created_at FROM articles WHERE id = $1",
            &article_id,
        );

        let result = self
            .pool
            .fetch_optional(query)
            .await
            .map_err(|err| GetArticleError::Repository(err.to_string()))?
            .map(|row| {
                Article::new(
                    row.get("id"),
                    row.get("author_id"),
                    row.get("title"),
                    row.get("content"),
                    "".into(),
                )
            });

        result.ok_or_else(|| GetArticleError::NotFound(article_id.to_string()))
    }
}
