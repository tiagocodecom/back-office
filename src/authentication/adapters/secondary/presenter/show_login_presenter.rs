use crate::authentication::adapters::secondary::presenter::show_login_view_model::FormViewModel;
use crate::authentication::application::ports::secondary::ForPresentingForm;
use crate::authentication::domain::form::Form;
use actix_web::web::Html;
use handlebars::Handlebars;
use std::sync::Arc;

pub struct ShowLoginPresenter<'a> {
    view: Arc<Handlebars<'a>>,
}

impl<'a> ShowLoginPresenter<'a> {
    fn new(view: &Arc<Handlebars<'a>>) -> Self {
        Self { view: view.clone() }
    }

    pub fn with_view(view: &Arc<Handlebars<'a>>, template: &str, context: Form) -> Html {
        Html::new(Self::new(view).render(template, context))
    }
}

impl<'a> ForPresentingForm for ShowLoginPresenter<'a> {
    fn render(&self, template: &str, context: Form) -> String {
        let context: FormViewModel = context.into();
        self.view.render(template, &context).unwrap()
    }
}
