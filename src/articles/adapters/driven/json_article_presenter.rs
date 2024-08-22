use crate::articles::adapters::driven::ArticleViewModel;
use crate::articles::domain::{Article, RenderOutput};
use crate::articles::ports::driven::RenderArticlePort;
use serde_json::json;

/// A presenter that renders an `Article` into a specific output format.
#[derive(Debug, Clone)]
pub struct ArticleJsonPresenter {}

impl ArticleJsonPresenter {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderArticlePort for &ArticleJsonPresenter {
    type Output = RenderOutput;

    /// Renders an `Article` as JSON within a `RenderOutput`.
    fn render_article(&self, article: Article) -> anyhow::Result<Self::Output> {
        let article = ArticleViewModel::from(article);
        let output = RenderOutput::Json(json!(article));

        Ok(output)
    }
}
