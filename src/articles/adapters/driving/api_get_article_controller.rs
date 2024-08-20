use crate::articles::application::services::GetArticleService;
use crate::framework::container::Container;
use actix_web::web::{Data, Path};
use actix_web::HttpResponse;
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
pub async fn handle(container: Data<Container>, path: Path<GetArticleRequestPath>) -> HttpResponse {
    let results = GetArticleService::with_repository(&container.article_repository, &path.id).await;

    match results {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
