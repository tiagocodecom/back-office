use crate::articles::adapters::driven::ArticleViewModel;
use crate::articles::domain::{Article, RenderOutput};
use crate::articles::ports::driven::RenderArticlePort;
use handlebars::Handlebars;
use std::sync::Arc;

/// A presenter that renders an `Article` as HTML using Handlebars templates.
#[derive(Debug, Clone)]
pub struct ArticleHtmlPresenter<'a> {
    view: Arc<Handlebars<'a>>,
}

impl<'a> ArticleHtmlPresenter<'a> {
    pub fn new(view: Arc<Handlebars<'a>>) -> Self {
        Self { view }
    }
}

impl RenderArticlePort for &ArticleHtmlPresenter<'_> {
    type Output = RenderOutput;

    /// Renders an `Article` into HTML using the specified template.
    fn render_article(&self, article: Article) -> anyhow::Result<Self::Output> {
        let article = ArticleViewModel::from(article);
        let output = RenderOutput::Html(self.view.render("articles/show", &article)?);

        Ok(output)
    }
}
