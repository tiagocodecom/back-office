use crate::authentication::application::ports::driving::ShowLoginService;
use crate::authentication::application::use_cases::ShowLoginUseCase;
use crate::authentication::entities::RenderOutput;
use crate::Container;
use actix_web::web::{Data, Html};
use actix_web::Responder;

#[tracing::instrument(name = "WebShowLoginController", skip(sc))]
pub async fn handle(sc: Data<Container<'_>>) -> impl Responder {
    let provider = &sc.default_login_provider;
    let presenter = &sc.html_login_presenter;

    let result = ShowLoginUseCase::new(provider, presenter)
        .execute()
        .await
        .unwrap();

    match result {
        RenderOutput::Html(html) => Html::new(html),
        _ => Html::new("Internal server error"),
    }
}
