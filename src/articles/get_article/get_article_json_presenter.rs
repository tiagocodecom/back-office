use crate::articles::get_article::{GetArticleViewModel, RenderArticlePort};
use crate::articles::{Article, RenderOutput};
use serde_json::json;

/// A presenter that renders an `Article` into a specific output format.
#[derive(Debug, Clone)]
pub struct GetArticleJsonPresenter {}

impl GetArticleJsonPresenter {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderArticlePort for &GetArticleJsonPresenter {
    type Output = RenderOutput;

    /// Renders an `Article` as JSON within a `RenderOutput`.
    fn render_article(&self, article: Article) -> anyhow::Result<Self::Output> {
        let article = GetArticleViewModel::from(article);
        let output = RenderOutput::Json(json!(article));

        Ok(output)
    }
}
