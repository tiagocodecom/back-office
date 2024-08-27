use crate::articles::get_article::GetArticleError;
use crate::articles::Article;
#[cfg(test)]
use mockall::automock;

/// A trait for rendering an article into a specific output format.
///
/// Implement this trait to convert an `Article` into a desired format, such as JSON or HTML.
#[cfg_attr(test, automock(type Output = crate::articles::RenderOutput;))]
pub trait RenderArticlePort {
    /// The output format type, like JSON or HTML.
    type Output;

    /// Renders an `Article` into the specified output format.
    ///
    /// # Parameters
    /// - `article`: The `Article` to render.
    ///
    /// # Returns
    /// A result containing the rendered output or an error if the process fails.
    fn render_article(&self, article: Article) -> Result<Self::Output, GetArticleError>;
}
