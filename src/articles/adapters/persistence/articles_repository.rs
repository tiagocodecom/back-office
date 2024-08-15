use crate::articles::application::ports::SaveArticleUseCase;
use crate::articles::domain::NewArticle;
use anyhow::Context;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::query;
use sqlx::{Executor, PgPool};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ArticleRepository {
    db_pool: Arc<PgPool>,
}

impl ArticleRepository {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}
#[async_trait(?Send)]
impl SaveArticleUseCase for ArticleRepository {
    #[tracing::instrument(name = "Store article in the database", skip(self, article))]
    async fn save_article(&self, article: &NewArticle) -> anyhow::Result<Uuid> {
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

        self.db_pool
            .execute(query)
            .await
            .context("Failed to save the article")?;

        Ok(article_id)
    }
}
