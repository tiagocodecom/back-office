use serde_json::Value;

/// Represents the different output formats that an article can be rendered into.
///
/// This enum is used to encapsulate the result of rendering an article, allowing for
/// various formats such as JSON and HTML. Each variant contains the rendered
/// data in the corresponding format.
///
/// # Variants
///
/// - `Json(Value)`: Represents the article rendered as a JSON object. The `Value`
///   type from `serde_json` is used to capture the structured JSON data.
/// - `Html(String)`: Represents the article rendered as an HTML string. The string
///   contains the HTML markup for the article.
///
/// # Examples
///
/// ```rust
/// use serde_json::json;
/// use back_office::articles::RenderOutput;
///
/// let json_output = RenderOutput::Json(json!({"title": "Example Article"}));
/// let html_output = RenderOutput::Html("<h1>Example Article</h1>".into());
/// ```

pub enum RenderOutput {
    Json(Value),
    Html(String),
}
