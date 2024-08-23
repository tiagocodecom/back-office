use crate::authentication::domain::Form;

/// A trait for rendering forms using a template and context.
pub trait RenderFormPort {
    /// The type of the output produced by the rendering process.
    type Output;

    /// Renders a form based on the given template and context.
    ///
    /// # Parameters
    /// - `template`: A string slice representing the template to use for rendering.
    /// - `context`: A `Form` instance providing the data to be used in the rendering.
    ///
    /// # Returns
    /// An `anyhow::Result` that contains the rendered form as `Self::Output` if successful, or an error if rendering fails.
    ///

    fn render_form(&self, form: Form) -> anyhow::Result<Self::Output>;
}
