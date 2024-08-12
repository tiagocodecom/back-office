use crate::articles::application::services::save_article_service;
use crate::articles::domain::NewArticle;
use crate::framework::container::Container;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct NewArticleRequest {
    // pub author_id: Uuid,
    pub title: String,
    pub content: String,
}

impl From<NewArticleRequest> for NewArticle {
    fn from(value: NewArticleRequest) -> Self {
        Self::new(Uuid::new_v4(), value.title, value.content)
    }
}

pub async fn save_article(
    container: Data<Container>,
    request: Json<NewArticleRequest>,
) -> HttpResponse {
    let request = request.into_inner();
    let results =
        save_article_service::execute(&container.article_repository, &request.into()).await;

    match results {
        Ok(article) => HttpResponse::Ok().body(article.id().to_string()),
        Err(_) => HttpResponse::Ok().body("error"),
    }
}
