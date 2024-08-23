use crate::articles::domain::RenderOutput;
use crate::articles::services::GetArticleService;
use crate::framework::container::Container;
use actix_web::web::{Data, Html, Path};
use actix_web::Responder;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct ShowArticleRequestPath {
    id: Uuid,
}

#[tracing::instrument(
    name = "Show article request",
    skip(container, path),
    fields(
        article_id = %path.id,
    )
)]
pub async fn handle(
    container: Data<Container<'_>>,
    path: Path<ShowArticleRequestPath>,
) -> impl Responder {
    let repository = &container.article_postgres_repository;
    let presenter = &container.article_html_presenter.clone();

    let result = GetArticleService::new(repository, presenter)
        .execute(path.id)
        .await
        .unwrap();

    match result {
        RenderOutput::Html(html) => Html::new(html),
        _ => Html::new("Internal server error"),
    }
}
