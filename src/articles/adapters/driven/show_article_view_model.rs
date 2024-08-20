use crate::articles::application::domain::Article;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ShowArticleViewModel {
    pub id: String,
    pub title: String,
    pub content: String,
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

impl From<Article> for ShowArticleViewModel {
    fn from(article: Article) -> Self {
        Self {
            id: article.id().to_string(),
            title: article.title().into(),
            content: article.content().into(),
            author_id: article.author_id().to_string(),
            created_at: "Just now".into(),
        }
    }
}
