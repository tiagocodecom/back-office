use crate::authentication::entities::AuthenticationError;
use async_trait::async_trait;

/// This trait defines the functionality for obtaining a login provider.
///
/// A login provider is a service responsible for handling user authentication.
/// It may integrate with third-party services (e.g., OAuth providers) or offer
/// a custom login interface, such as an HTML form-based authentication.
#[async_trait(?Send)]
pub trait LoginProvider {
    type Output;

    async fn get_login(&self) -> Result<Self::Output, AuthenticationError>;
}
