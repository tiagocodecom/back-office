use crate::authentication::domain::RenderOutput;
use crate::authentication::ports::driven::{FetchFormPort, RenderFormPort};

/// A service for showing a form by delegating rendering to a presenter.
pub struct GetFormService<R, P> {
    repository: R,
    presenter: P,
}

impl<R, P> GetFormService<R, P>
where
    R: FetchFormPort,
    P: RenderFormPort<Output = RenderOutput>,
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
    /// This method calls the `render_form` method on the `presenter` and handles the result.
    ///
    /// # Returns
    /// An `anyhow::Result` indicating success (`Ok(())`) or failure (`Err`) of the operation.
    pub async fn execute(&self, form_id: &str) -> anyhow::Result<RenderOutput> {
        let form = self.repository.get_form_by_id(form_id).await?;
        let result = self.presenter.render_form(form)?;

        Ok(result)
    }
}
