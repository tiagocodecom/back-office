use crate::authentication::get_login::Form;
use async_trait::async_trait;

/// A trait for fetching a form by its unique ID.
#[async_trait(?Send)]
pub trait GetLoginPort {
    /// Retrieves an `Form` by its ID.
    ///
    /// # Parameters
    /// - `form_if`: The unique identifier of the form.
    ///
    /// # Returns
    /// A result containing the `Form` or an error if not found.
    async fn get_login(&self) -> anyhow::Result<Form>;
}
