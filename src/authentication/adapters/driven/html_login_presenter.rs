use crate::authentication::application::ports::driven::LoginPresenter;
use crate::authentication::entities::{AuthenticationError, LoginProviderOutput, RenderOutput};
use crate::common::adapters::driven::FormViewModel;
use handlebars::Handlebars;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct HtmlLoginPresenter<'a> {
    view: Arc<Handlebars<'a>>,
}

impl<'a> HtmlLoginPresenter<'a> {
    pub fn new(view: Arc<Handlebars<'a>>) -> Self {
        Self { view }
    }
}

impl<'a> LoginPresenter for &HtmlLoginPresenter<'a> {
    type Input = LoginProviderOutput;
    type Output = RenderOutput;

    fn present_login(&self, login: Self::Input) -> Result<Self::Output, AuthenticationError> {
        match login {
            LoginProviderOutput::Default(form) => {
                let context = FormViewModel::from(form);
                let output = self
                    .view
                    .render("auth/login", &context)
                    .map_err(|e| AuthenticationError::Unexpected(e.to_string()))?;

                Ok(RenderOutput::Html(output))
            }
        }
    }
}
