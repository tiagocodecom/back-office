use crate::articles::application::ports::for_getting_articles::ForGettingArticles;
use crate::articles::domain::article::{Article, ArticlesList};
use std::fmt::Error;

#[derive(Debug, Clone)]
pub struct ArticlesRepository {}

#[async_trait::async_trait(?Send)]
impl ForGettingArticles for ArticlesRepository {
    async fn find_article(&self) -> Result<Article, Error> {
        todo!()
    }

    async fn find_articles(&self) -> Result<ArticlesList, Error> {
        todo!()
    }
}
