use crate::authentication::domain::Form;
use async_trait::async_trait;

/// A trait for fetching a form by its unique ID.
#[async_trait(?Send)]
pub trait FetchFormPort {
    /// Retrieves an `Form` by its ID.
    ///
    /// # Parameters
    /// - `form_if`: The unique identifier of the form.
    ///
    /// # Returns
    /// A result containing the `Form` or an error if not found.
    async fn get_form_by_id(&self, form_id: &str) -> anyhow::Result<Form>;
}
