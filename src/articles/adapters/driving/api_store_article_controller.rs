use crate::articles::application::ports::driving::StoreArticleService;
use crate::articles::application::use_cases::StoreArticleUseCase;
use crate::articles::entities::{NewArticle, RenderOutput};
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
    name = "ApiStoreArticleController",
    skip(sc, request),
    fields(
        title = %request.title,
        author_id = %request.author_id
    )
)]
pub async fn handle(sc: Data<Container<'_>>, request: Json<NewArticleRequest>) -> HttpResponse {
    match request.validate() {
        Ok(_) => (),
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    }

    let repository = &sc.postgres_article_repository;
    let presenter = &sc.json_get_article_presenter;
    let result = StoreArticleUseCase::new(repository, presenter)
        .execute(&request.into_inner().into())
        .await
        .unwrap();

    match result {
        RenderOutput::Json(json) => HttpResponse::Ok().json(json),
        _ => HttpResponse::InternalServerError().finish(),
    }
}
