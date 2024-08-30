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
/// use back_office::articles::entities::{Article, RenderOutput};
///
/// let json_output = RenderOutput::Json(json!({"title": "Example Article"}));
/// let html_output = RenderOutput::Html("<h1>Example Article</h1>".into());
/// ```

#[derive(Debug, PartialEq)]
pub enum RenderOutput {
    Json(Value),
    Html(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn creates_a_json_variant() {
        let json = json!({"title": "Example Article"});
        let output = RenderOutput::Json(json.clone());

        match output {
            RenderOutput::Json(value) => assert_eq!(value, json),
            _ => panic!("Expected a Json variant"),
        }
    }

    #[test]
    fn creates_an_html_variant() {
        let html = "<h1>Example Article</h1>".to_string();
        let output = RenderOutput::Html(html.clone());

        match output {
            RenderOutput::Html(value) => assert_eq!(value, html),
            _ => panic!("Expected an Html variant"),
        }
    }
}
