use crate::articles::get_article::GetArticleService;
use crate::articles::RenderOutput;
use crate::framework::container::Container;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, Responder};
use uuid::Uuid;

#[derive(validator::Validate, serde::Deserialize)]
pub struct GetArticleRequestPath {
    id: Uuid,
}

#[tracing::instrument(
    name = "Get article request",
    skip(container, path),
    fields(
        article_id = %path.id,
    )
)]
pub async fn handle(
    container: Data<Container<'_>>,
    path: Path<GetArticleRequestPath>,
) -> impl Responder {
    let repository = &container.article_postgres_repository;
    let presenter = &container.article_json_presenter;

    let result = GetArticleService::new(repository, presenter)
        .execute(path.id)
        .await
        .unwrap();

    match result {
        RenderOutput::Json(json) => HttpResponse::Ok().json(json),
        _ => HttpResponse::InternalServerError().finish(),
    }
}
