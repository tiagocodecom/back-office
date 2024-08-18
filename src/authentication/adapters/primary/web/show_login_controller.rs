use crate::authentication::adapters::secondary::presenter::ShowLoginPresenter;
use crate::authentication::domain::form::{Form, FormButton, FormInput};
use crate::Container;
use actix_web::web::Data;
use actix_web::Responder;

#[tracing::instrument(name = "Show login request", skip(container))]
pub async fn show_login(container: Data<Container>) -> impl Responder {
    let context = Form::new(
        String::from("Tiagocode Admin"),
        vec![
            FormInput::new(
                String::from("username"),
                String::from("username"),
                String::from(""),
                String::from("Username"),
                String::from("text"),
            ),
            FormInput::new(
                String::from("password"),
                String::from("password"),
                String::from(""),
                String::from("Password"),
                String::from("password"),
            ),
        ],
        FormButton::new(
            String::from("submit"),
            String::from("Login"),
            String::from("submit"),
        ),
    );
    ShowLoginPresenter::with_view(&container.view, "auth/login", context)
}
