use crate::articles::application::ports::ForGettingAuthor;
use crate::articles::domain::Author;
use uuid::Uuid;

pub async fn execute(
    driven_port: &impl ForGettingAuthor,
    author_id: Uuid,
) -> anyhow::Result<Option<Author>> {
    driven_port.find_author(author_id).await
}
