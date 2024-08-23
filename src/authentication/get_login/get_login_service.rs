use crate::authentication::get_login::{GetLoginPort, RenderLoginPort};
use crate::authentication::RenderOutput;

/// A service for showing a form by delegating rendering to a presenter.
pub struct GetLoginService<R, P> {
    repository: R,
    presenter: P,
}

impl<R, P> GetLoginService<R, P>
where
    R: GetLoginPort,
    P: RenderLoginPort<Output = RenderOutput>,
{
    /// Creates a new `ShowFormService` with the specified presenter.
    ///
    /// # Parameters
    /// - `presenter`: An instance of a type implementing the `RenderFormPort` trait.
    ///
    /// # Returns
    /// A new instance of `ShowFormService`.
    pub fn new(repository: R, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    /// Executes the service to render the form using the presenter.
    ///
    /// This method calls the `render_login` method on the `presenter` and handles the result.
    ///
    /// # Returns
    /// An `anyhow::Result` indicating success (`Ok(())`) or failure (`Err`) of the operation.
    pub async fn execute(&self) -> anyhow::Result<RenderOutput> {
        let login = self.repository.get_login().await?;
        let result = self.presenter.render_login(login)?;

        Ok(result)
    }
}
