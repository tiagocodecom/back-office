use crate::articles::application::services::save_article_service;
use crate::articles::domain::NewArticle;
use crate::framework::container::Container;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use uuid::Uuid;
use validator::Validate;

#[derive(validator::Validate, serde::Deserialize)]
pub struct NewArticleRequest {
    pub author_id: Uuid,
    #[validate(length(min = 10))]
    pub title: String,
    #[validate(length(min = 100))]
    pub content: String,
}

impl From<NewArticleRequest> for NewArticle {
    fn from(value: NewArticleRequest) -> Self {
        Self::new(value.author_id, value.title, value.content)
    }
}

#[tracing::instrument(
    name = "Save new article request",
    skip(container, request),
    fields(
        author_id = %request.author_id,
        title = %request.title
    )
)]
pub async fn save_article(
    container: Data<Container>,
    request: Json<NewArticleRequest>,
) -> HttpResponse {
    match request.validate() {
        Ok(_) => (),
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    }

    let request = request.into_inner();
    let results =
        save_article_service::execute(&container.article_repository, &request.into()).await;

    match results {
        Ok(article_id) => HttpResponse::Ok().body(article_id.to_string()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
