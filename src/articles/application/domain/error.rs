#[derive(thiserror::Error, Debug)]
pub enum ArticleError {
    #[error("{0}")]
    ValidationError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Article not found: {0}")]
    NotFound(String),

    #[error("Invalid author: {0}")]
    InvalidAuthor(String),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
