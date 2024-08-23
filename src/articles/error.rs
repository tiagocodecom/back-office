#[derive(thiserror::Error, Debug)]
pub enum ArticleError {
    /// Represents a validation error with a custom message.
    #[error("{0}")]
    ValidationError(String),

    /// Represents a database error, wrapping an `sqlx::Error`.
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    /// Indicates that an article was not found, with the missing article's identifier or description.
    #[error("Article not found: {0}")]
    NotFound(String),

    /// Indicates that an invalid author was specified, with details provided in the message.
    #[error("Invalid author: {0}")]
    InvalidAuthor(String),

    /// Represents an error during the rendering process, wrapping an `anyhow::Error`.
    // #[error(transparent)]
    // RenderError(#[from] anyhow::Error),

    /// Represents an unexpected error, wrapping an `anyhow::Error`.
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
