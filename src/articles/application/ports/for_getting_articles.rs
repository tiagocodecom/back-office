use crate::articles::domain::article::{Article, ArticlesList};
use async_trait::async_trait;
use std::fmt::Error;

#[async_trait(?Send)]
pub trait ForGettingArticles {
    async fn find_article(&self) -> Result<Article, Error>;

    async fn find_articles(&self) -> Result<ArticlesList, Error>;
}
