use crate::authentication::domain::RenderOutput;
use crate::authentication::services::get_form_service::GetFormService;
use crate::Container;
use actix_web::web::{Data, Html, Path};
use actix_web::Responder;
use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct GetFormRequestPath {
    form_id: String,
}

#[tracing::instrument(name = "Get form request", skip(container, path))]
pub async fn handle(
    container: Data<Container<'_>>,
    path: Path<GetFormRequestPath>,
) -> impl Responder {
    let repository = &container.auth_form_memory_repository;
    let presenter = &container.auth_form_html_presenter;
    let form_id = format!("form_{}", path.form_id);

    let result = GetFormService::new(repository, presenter)
        .execute(&form_id)
        .await
        .unwrap();

    match result {
        RenderOutput::Html(html) => Html::new(html),
        _ => Html::new("Internal server error"),
    }
}
