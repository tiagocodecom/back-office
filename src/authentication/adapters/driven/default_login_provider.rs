use crate::authentication::application::ports::driven::LoginProvider;
use crate::authentication::entities::{AuthenticationError, FormMethod, LoginProviderOutput};
use crate::common::entities::{Form, FormButton, FormInput};
use async_trait::async_trait;

#[derive(Debug, Clone, Default)]
pub struct DefaultLoginProvider {}

#[async_trait(?Send)]
impl LoginProvider for &DefaultLoginProvider {
    type Output = LoginProviderOutput;

    async fn get_login(&self) -> Result<Self::Output, AuthenticationError> {
        let form = Form::builder()
            .id("login-form".into())
            .method(FormMethod::Post)
            .action("/auth/login".into())
            .title("Welcome to Tiagocode".into())
            .fields(vec![
                FormInput::builder()
                    .id("email".into())
                    .title("Email".into())
                    .placeholder("jonhdoe@gmail.com".into())
                    .required(true)
                    .build()
                    .map_err(|e| AuthenticationError::Unexpected(e.to_string()))?,
                FormInput::builder()
                    .id("password".into())
                    .title("Password".into())
                    .input_type("password".into())
                    .placeholder("*************".into())
                    .required(true)
                    .build()
                    .map_err(|e| AuthenticationError::Unexpected(e.to_string()))?,
            ])
            .submit(
                FormButton::builder()
                    .id("submit".into())
                    .title("Login".into())
                    .build()
                    .map_err(|e| AuthenticationError::Unexpected(e.to_string()))?,
            )
            .build()
            .map_err(|e| AuthenticationError::Unexpected(e.to_string()))?;

        Ok(LoginProviderOutput::Default(form))
    }
}
