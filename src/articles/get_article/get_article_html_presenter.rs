use crate::articles::get_article::{GetArticleError, GetArticleViewModel, RenderArticlePort};
use crate::articles::{Article, RenderOutput};
use handlebars::Handlebars;
use std::sync::Arc;

/// A presenter that renders an `Article` as HTML using Handlebars templates.
#[derive(Debug, Clone)]
pub struct GetArticleHtmlPresenter<'a> {
    view: Arc<Handlebars<'a>>,
}

impl<'a> GetArticleHtmlPresenter<'a> {
    pub fn new(view: Arc<Handlebars<'a>>) -> Self {
        Self { view }
    }
}

impl RenderArticlePort for &GetArticleHtmlPresenter<'_> {
    type Output = RenderOutput;

    /// Renders an `Article` into HTML using the specified template.
    fn render_article(&self, article: Article) -> Result<Self::Output, GetArticleError> {
        let article = GetArticleViewModel::from(article);
        let output = self
            .view
            .render("articles/show", &article)
            .map_err(|error| GetArticleError::Presenter(error.to_string()))?;

        Ok(RenderOutput::Html(output))
    }
}
