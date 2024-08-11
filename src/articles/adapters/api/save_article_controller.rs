use crate::articles::application::services::save_article_service;
use crate::articles::domain::NewArticle;
use crate::container::Container;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct NewArticleRequest {
    pub author_id: Uuid,
    pub title: String,
    pub text: String,
}

impl From<NewArticleRequest> for NewArticle {
    fn from(value: NewArticleRequest) -> Self {
        Self::new(value.author_id, value.title, value.text)
    }
}

pub async fn save_article(
    container: Data<Container>,
    request: Json<NewArticleRequest>,
    db_pool: Data<PgPool>,
) -> impl actix_web::Responder {
    let request = request.into_inner();
    let results =
        save_article_service::execute(&db_pool, &container.article_repository, &request.into())
            .await;

    match results {
        Ok(_article_id) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
