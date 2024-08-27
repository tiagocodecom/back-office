use crate::articles::get_article::{GetArticleError, GetArticleViewModel, RenderArticlePort};
use crate::articles::{Article, RenderOutput};

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
    fn render_article(&self, article: Article) -> Result<Self::Output, GetArticleError> {
        let article = GetArticleViewModel::from(article);
        let output = serde_json::json!(article);

        Ok(RenderOutput::Json(output))
    }
}
