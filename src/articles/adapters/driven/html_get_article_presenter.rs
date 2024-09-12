use crate::articles::adapters::driven::get_article_view_model::GetArticleViewModel;
use crate::articles::application::ports::driven::GetArticlePresenter;
use crate::articles::entities::{Article, Error, RenderOutput};
use handlebars::Handlebars;
use std::sync::Arc;

/// A presenter that renders an `Article` as HTML using Handlebars templates.
#[derive(Debug, Clone)]
pub struct HtmlGetArticlePresenter<'a> {
    view: Arc<Handlebars<'a>>,
}

impl<'a> HtmlGetArticlePresenter<'a> {
    pub fn new(view: Arc<Handlebars<'a>>) -> Self {
        Self { view }
    }
}

impl GetArticlePresenter for &HtmlGetArticlePresenter<'_> {
    type Output = RenderOutput;

    fn present_article(&self, article: Article) -> Result<Self::Output, Error> {
        let article = GetArticleViewModel::from(article);

        let output = self
            .view
            .render("articles/show", &article)
            .map_err(|error| Error::Presenter(error.to_string()))?;

        Ok(RenderOutput::Html(output))
    }
}
