use crate::authentication::get_login::{Form, GetLoginViewModel, RenderLoginPort};
use crate::authentication::RenderOutput;
use handlebars::Handlebars;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GetLoginHtmlPresenter<'a> {
    view: Arc<Handlebars<'a>>,
}

impl<'a> GetLoginHtmlPresenter<'a> {
    pub fn new(view: Arc<Handlebars<'a>>) -> Self {
        Self { view }
    }
}

impl<'a> RenderLoginPort for &GetLoginHtmlPresenter<'a> {
    type Output = RenderOutput;

    fn render_login(&self, form: Form) -> anyhow::Result<Self::Output> {
        let form = GetLoginViewModel::from(form);
        let html = self.view.render("auth/login", &form)?;

        Ok(RenderOutput::Html(html))
    }
}
