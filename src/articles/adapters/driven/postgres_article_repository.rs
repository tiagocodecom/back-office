use crate::articles::application::ports::driven::{GetArticleRepository, StoreArticleRepository};
use crate::articles::entities::{Article, ArticleBuilder, Error, NewArticle, Slug};
use async_trait::async_trait;
use sqlx::query;
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
    async fn store_article(&self, new_article: &NewArticle) -> Result<Uuid, Error> {
        let article_id = Uuid::new_v4();

        let query = query!(
            r#"
                INSERT INTO articles (
                    id,
                    slug,
                    title,
                    summary,
                    content,
                    thumbnail_uri,
                    author_id
                ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            &article_id,
            &new_article.slug(),
            &new_article.title(),
            &new_article.summary(),
            &new_article.content(),
            "/static/images/articles/thumbnail.jpg",
            &new_article.author_id()
        );

        self.pool
            .execute(query)
            .await
            .map_err(|e| Error::Persistence(e.to_string()))?;

        Ok(article_id)
    }
}

#[async_trait(?Send)]
impl GetArticleRepository for &PostgresArticleRepository {
    #[tracing::instrument(name = "Get article by ID", skip(self, article_id))]
    async fn get_article_by_id(&self, article_id: &Uuid) -> Result<Article, Error> {
        let article = query!(
            r#"
                SELECT
                    id,
                    likes,
                    slug,
                    title,
                    summary,
                    content,
                    author_id,
                    thumbnail_uri,
                    status as "status: String",
                    created_at,
                    updated_at
                FROM
                    articles
                WHERE
                    id = $1
            "#,
            article_id,
        )
        .fetch_one(&*self.pool)
        .await
        .map(|row| {
            ArticleBuilder::default()
                .id(row.id)
                .likes(row.likes)
                .title(row.title)
                .slug(Slug::new(row.slug))
                .summary(row.summary)
                .content(row.content)
                .author_id(row.author_id)
                .thumbnail_uri(row.thumbnail_uri)
                .status(row.status.as_str().into())
                .created_at(row.created_at)
                .updated_at(row.updated_at)
                .build()
                .unwrap()
        })
        .map_err(|e| Error::Persistence(e.to_string()))?;

        Ok(article)
    }
}
