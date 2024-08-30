use crate::articles::adapters::driven::get_article_view_model::GetArticleViewModel;
use crate::articles::application::ports::driven::GetArticlePresenter;
use crate::articles::entities::{Article, ArticleError, RenderOutput};

/// A presenter that renders an `Article` into a specific output format.
#[derive(Debug, Clone)]
pub struct JsonGetArticlePresenter {}

impl JsonGetArticlePresenter {
    pub fn new() -> Self {
        Self {}
    }
}

impl GetArticlePresenter for &JsonGetArticlePresenter {
    type Output = RenderOutput;

    fn present_article(&self, article: Article) -> Result<Self::Output, ArticleError> {
        let article = GetArticleViewModel::from(article);
        let output = serde_json::json!(article);

        Ok(RenderOutput::Json(output))
    }
}
