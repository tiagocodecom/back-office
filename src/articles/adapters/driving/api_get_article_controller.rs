use crate::articles::application::ports::driving::GetArticleService;
use crate::articles::application::use_cases::GetArticleUseCase;
use crate::articles::entities::RenderOutput;
use crate::framework::container::Container;
use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct GetArticleUriPath {
    id: Uuid,
}

#[instrument(
    name = "ApiGetArticleController",
    skip(sc, path),
    fields(article_id = %path.id)
)]
pub async fn handle(sc: Data<Container<'_>>, path: Path<GetArticleUriPath>) -> impl Responder {
    let repository = &sc.postgres_article_repository;
    let presenter = &sc.json_get_article_presenter;

    let result = GetArticleUseCase::new(repository, presenter)
        .execute(&path.id)
        .await
        .unwrap();

    match result {
        RenderOutput::Json(json) => HttpResponse::Ok().json(json),
        _ => HttpResponse::InternalServerError().finish(),
    }
}
