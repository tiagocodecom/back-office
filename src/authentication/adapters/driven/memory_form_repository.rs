use crate::authentication::domain::{Form, FormButton, FormInput};
use crate::authentication::ports::driven::FetchFormPort;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct AuthFormMemoryRepository {}

impl AuthFormMemoryRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl FetchFormPort for &AuthFormMemoryRepository {
    async fn get_form_by_id(&self, form_id: &str) -> anyhow::Result<Form> {
        match form_id {
            "form_login" => Ok(Form::new(
                "Welcome to Tiagocode".into(),
                vec![
                    FormInput::new(
                        "email".to_string(),
                        "email".to_string(),
                        "".to_string(),
                        "Email".to_string(),
                        "email".to_string(),
                    ),
                    FormInput::new(
                        "password".to_string(),
                        "password".to_string(),
                        "".to_string(),
                        "Password".to_string(),
                        "password".to_string(),
                    ),
                ],
                FormButton::new(
                    "submit".to_string(),
                    "Submit".to_string(),
                    "submit".to_string(),
                ),
            )),
            _ => Err(anyhow::anyhow!("Form not found")),
        }
    }
}
