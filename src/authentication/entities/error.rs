#[derive(Debug)]
pub enum AuthenticationError {
    InvalidCredentials,
    Unexpected(String),
}
