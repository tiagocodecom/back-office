use crate::articles::application::ports::SaveArticleUseCase;
use crate::articles::domain::{Article, Author, NewArticle};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
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
    async fn save_article(&self, article: &NewArticle) -> anyhow::Result<Article> {
        let article_id = Uuid::new_v4();
        let created_at = Utc::now();
        // let query = query!(
        //     "INSERT INTO articles (id, author_id, title, content, created_at) VALUES ($1, $2, $3, $4, $5)",
        //     &article_id,
        //     &article.author_id(),
        //     &article.title(),
        //     &article.content(),
        //     &created_at,
        // );

        // self.db_pool.execute(query).await?;

        Ok(Article::new(
            article_id,
            Author::new(
                article.author_id().clone(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ),
            article.title().clone().into(),
            article.content().clone().into(),
            created_at.to_string(),
        ))
    }
}
