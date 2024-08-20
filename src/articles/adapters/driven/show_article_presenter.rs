use crate::articles::adapters::driven::show_article_view_model::ShowArticleViewModel;
use crate::articles::application::domain::Article;
use crate::articles::application::ports::driven::DisplayArticlePort;
use actix_web::web::Html;
use handlebars::Handlebars;
use std::sync::Arc;

pub struct ShowArticlePresenter<'a> {
    view: Arc<Handlebars<'a>>,
}

impl<'a> ShowArticlePresenter<'a> {
    pub fn new(view: &Arc<Handlebars<'a>>) -> Self {
        Self { view: view.clone() }
    }

    pub fn with_view(view: &Arc<Handlebars<'a>>, article: Article) -> Html {
        Html::new(Self::new(view).render(article))
    }
}

impl<'a> DisplayArticlePort for ShowArticlePresenter<'a> {
    fn render(&self, article: Article) -> String {
        let article: ShowArticleViewModel = article.into();
        self.view.render("articles/show", &article).unwrap()
    }
}
