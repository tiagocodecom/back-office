use crate::authentication::entities::AuthenticationError;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait ShowLoginService {
    type Output;

    async fn execute(&self) -> Result<Self::Output, AuthenticationError>;
}
