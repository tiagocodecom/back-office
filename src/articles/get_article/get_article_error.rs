use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum GetArticleError {
    #[error("Not found `{0}`")]
    NotFound(String),

    #[error("Unexpected `{0}`")]
    Unexpected(String),

    #[error("Repository `{0}`")]
    Repository(String),

    #[error("Presenter `{0}`")]
    Presenter(String),
}
