use async_trait::async_trait;

/// A trait for handling the use case of showing a login form.
#[async_trait(?Send)]
pub trait ShowLoginFormUseCase {
    /// Executes the use case of showing the login form.
    ///
    /// # Returns
    /// An `anyhow::Result` indicating success (`Ok(())`) or failure (`Err`) of the operation.
    async fn execute(&self) -> anyhow::Result<()>;
}
