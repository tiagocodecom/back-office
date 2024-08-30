use crate::articles::application::ports::driven::{GetArticleRepository, StoreArticleRepository};
use crate::articles::entities::{Article, ArticleError, NewArticle};
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
impl StoreArticleRepository for &PostgresArticleRepository {
    #[tracing::instrument(
        name = "Storing a new article using the Postgres repository",
        skip(self, new_article)
    )]
    async fn store_article(&self, new_article: &NewArticle) -> Result<Uuid, ArticleError> {
        let article_id = Uuid::new_v4();
        let created_at = Utc::now();

        let query = query!(
            "INSERT INTO articles (id, author_id, title, content, created_at) VALUES ($1, $2, $3, $4, $5)",
            &article_id,
            &new_article.author_id(),
            &new_article.title(),
            &new_article.content(),
            &created_at,
        );

        self.pool
            .execute(query)
            .await
            .map_err(|e| ArticleError::Persistence(e.to_string()))?;

        Ok(article_id)
    }
}

#[async_trait(?Send)]
impl GetArticleRepository for &PostgresArticleRepository {
    #[tracing::instrument(
        name = "Searching an existing article by the ID",
        skip(self, article_id)
    )]
    async fn get_article_by_id(&self, article_id: &Uuid) -> Result<Article, ArticleError> {
        let result = self
            .pool
            .fetch_optional(query!(
                "SELECT id, author_id, title, content, created_at FROM articles WHERE id = $1",
                article_id,
            ))
            .await
            .map_err(|e| ArticleError::Persistence(e.to_string()))?
            .map(|row| {
                Article::new(
                    row.get("id"),
                    row.get("author_id"),
                    row.get("title"),
                    row.get("content"),
                    "".into(),
                )
            });

        match result {
            Some(article) => Ok(article),
            _ => return Err(ArticleError::NotFound("Not found article".into())),
        }
    }
}
