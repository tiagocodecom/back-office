use crate::authentication::application::ports::driven::{LoginPresenter, LoginProvider};
use crate::authentication::application::ports::driving::ShowLoginService;
use crate::authentication::entities::{AuthenticationError, LoginProviderOutput, RenderOutput};
use async_trait::async_trait;

/// A service for showing a form by delegating rendering to a presenter.
pub struct ShowLoginUseCase<R, P> {
    provider: R,
    presenter: P,
}

impl<R, P> ShowLoginUseCase<R, P>
where
    R: LoginProvider,
    P: LoginPresenter,
{
    pub fn new(provider: R, presenter: P) -> Self {
        Self {
            provider,
            presenter,
        }
    }
}

#[async_trait(?Send)]
impl<R, P> ShowLoginService for ShowLoginUseCase<R, P>
where
    R: LoginProvider<Output = LoginProviderOutput>,
    P: LoginPresenter<Input = LoginProviderOutput, Output = RenderOutput>,
{
    type Output = RenderOutput;

    async fn execute(&self) -> Result<Self::Output, AuthenticationError> {
        let login = self.provider.get_login().await?;

        self.presenter.present_login(login)
    }
}
