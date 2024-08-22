use crate::authentication::ports::driven::RenderFormPort;

use crate::authentication::adapters::driven::FormViewModel;
use crate::authentication::domain::{Form, RenderOutput};
use handlebars::Handlebars;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AuthFormHtmlPresenter<'a> {
    view: Arc<Handlebars<'a>>,
}

impl<'a> AuthFormHtmlPresenter<'a> {
    pub fn new(view: Arc<Handlebars<'a>>) -> Self {
        Self { view }
    }
}

impl<'a> RenderFormPort for &AuthFormHtmlPresenter<'a> {
    type Output = RenderOutput;

    fn render_form(&self, form: Form) -> anyhow::Result<Self::Output> {
        let form = FormViewModel::from(form);
        let html = self.view.render("auth/login", &form)?;

        Ok(RenderOutput::Html(html))
    }
}
