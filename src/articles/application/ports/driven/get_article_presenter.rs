use crate::articles::entities::{Article, ArticleError};
#[cfg(test)]
use mockall::automock;

/// A trait for rendering an article into a specific output format.
///
/// Implement this trait to convert an `Article` into a desired format, such as JSON or HTML.
#[cfg_attr(test, automock(type Output = crate::articles::entities::RenderOutput;))]
pub trait GetArticlePresenter {
    /// The output format type, like JSON or HTML.
    type Output;

    /// Renders an `Article` into the specified output format.
    ///
    /// # Parameters
    /// - `article`: The `Article` to render.
    ///
    /// # Returns
    /// A result containing the rendered output or an error if the process fails.
    fn present_article(&self, article: Article) -> Result<Self::Output, ArticleError>;
}
