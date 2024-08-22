/// Represents the different output formats that a form can be rendered into.
///
/// This enum is used to encapsulate the result of rendering an article, allowing for
/// various formats such as JSON and HTML. Each variant contains the rendered
/// data in the corresponding format.
///
/// # Variants

/// - `Html(String)`: Represents the article rendered as an HTML string. The string
///   contains the HTML markup for the article.

pub enum RenderOutput {
    Html(String),
}
