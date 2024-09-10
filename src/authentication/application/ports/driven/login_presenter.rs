use crate::authentication::entities::AuthenticationError;

pub trait LoginPresenter {
    type Input;
    type Output;

    fn present_login(&self, login: Self::Input) -> Result<Self::Output, AuthenticationError>;
}
