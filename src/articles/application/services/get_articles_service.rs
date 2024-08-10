use crate::articles::application::ports::for_getting_articles::ForGettingArticles;
use crate::articles::domain::article::ArticlesList;
use std::fmt::Error;

pub async fn execute(driven_port: &impl ForGettingArticles) -> Result<ArticlesList, Error> {
    driven_port.find_articles().await
}
