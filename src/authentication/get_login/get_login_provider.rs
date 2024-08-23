use crate::authentication::get_login::{Form, FormButton, FormInput, GetLoginPort};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct GetLoginProvider {}

impl GetLoginProvider {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl GetLoginPort for &GetLoginProvider {
    async fn get_login(&self) -> anyhow::Result<Form> {
        Ok(Form::new(
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
        ))
    }
}
