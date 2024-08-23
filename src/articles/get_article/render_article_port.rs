use crate::articles::Article;

/// A trait for rendering an article into a specific output format.
///
/// Implement this trait to convert an `Article` into a desired format, such as JSON or HTML.
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
    fn render_article(&self, article: Article) -> anyhow::Result<Self::Output>;
}
