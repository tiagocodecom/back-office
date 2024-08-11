use crate::articles::application::ports::ForGettingArticles;
use crate::articles::domain::ArticlesList;

pub async fn execute(driven_port: &impl ForGettingArticles) -> anyhow::Result<ArticlesList> {
    driven_port.find_articles().await
}
