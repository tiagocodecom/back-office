use crate::articles::adapters::driven::ShowArticlePresenter;
use crate::articles::application::services::GetArticleService;
use crate::framework::container::Container;
use actix_web::web::{Data, Html, Path};
use actix_web::Responder;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct ShowArticlePath {
    id: Uuid,
}

#[tracing::instrument(
    name = "Show article request",
    skip(container, path),
    fields(
        article_id = %path.id,
    )
)]
pub async fn handle(container: Data<Container>, path: Path<ShowArticlePath>) -> impl Responder {
    let result = GetArticleService::with_repository(&container.article_repository, &path.id).await;

    match result {
        Ok(article) => ShowArticlePresenter::with_view(&container.view, article),
        Err(_) => Html::new("Article not found"),
    }
}
