#[derive(thiserror::Error, Debug)]
pub enum ArticleError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
