use crate::articles::application::ports::driving::GetArticleService;
use crate::articles::application::use_cases::GetArticleUseCase;
use crate::articles::entities::RenderOutput;
use crate::framework::container::Container;
use actix_web::web::{Data, Html, Path};
use actix_web::Responder;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct GetArticleRequestPath {
    id: Uuid,
}

#[tracing::instrument(
    name = "WebGetArticleController",
    skip(sc, path),
    fields(article_id = %path.id)
)]
pub async fn handle(sc: Data<Container<'_>>, path: Path<GetArticleRequestPath>) -> impl Responder {
    let repository = &sc.postgres_article_repository;
    let presenter = &sc.html_get_article_presenter.clone();

    let result = GetArticleUseCase::new(repository, presenter)
        .execute(&path.id)
        .await
        .unwrap();

    match result {
        RenderOutput::Html(html) => Html::new(html),
        _ => Html::new("Internal server error"),
    }
}
