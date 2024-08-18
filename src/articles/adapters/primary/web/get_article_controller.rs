use crate::articles::adapters::secondary::presenter::show_article_presenter::ShowArticlePresenter;
use crate::articles::application::services::GetArticleService;
use crate::framework::container::Container;
use actix_web::web::{Data, Html, Path};
use actix_web::Responder;
use uuid::Uuid;

#[derive(validator::Validate, serde::Deserialize)]
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
pub async fn show_article(
    container: Data<Container>,
    path: Path<ShowArticleRequestPath>,
) -> impl Responder {
    let result = GetArticleService::with_repository(&container.article_repository, &path.id).await;

    match result {
        Ok(article) => ShowArticlePresenter::with_view(&container.view, article),
        Err(_) => Html::new("Article not found"),
    }
}
