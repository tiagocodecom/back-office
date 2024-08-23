use crate::articles::create_article::{CreateArticleService, NewArticle};
use crate::framework::container::Container;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Validate, Deserialize)]
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
    name = "Create article request",
    skip(container, request),
    fields(
        title = %request.title,
        author_id = %request.author_id
    )
)]
pub async fn handle(
    container: Data<Container<'_>>,
    request: Json<NewArticleRequest>,
) -> HttpResponse {
    match request.validate() {
        Ok(_) => (),
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    }

    let results = CreateArticleService::with_repository(
        &container.article_postgres_repository,
        &request.into_inner().into(),
    )
    .await;

    match results {
        Ok(article_id) => HttpResponse::Ok().body(article_id.to_string()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
