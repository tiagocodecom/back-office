use crate::authentication::get_login::GetLoginService;
use crate::authentication::RenderOutput;
use crate::Container;
use actix_web::web::{Data, Html};
use actix_web::Responder;

#[tracing::instrument(name = "Get form request", skip(container))]
pub async fn handle(container: Data<Container<'_>>) -> impl Responder {
    let repository = &container.login_provider;
    let presenter = &container.login_html_presenter;

    let result = GetLoginService::new(repository, presenter)
        .execute()
        .await
        .unwrap();

    match result {
        RenderOutput::Html(html) => Html::new(html),
        _ => Html::new("Internal server error"),
    }
}
