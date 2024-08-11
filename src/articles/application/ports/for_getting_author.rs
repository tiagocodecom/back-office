use crate::articles::domain::Author;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait ForGettingAuthor {
    async fn find_author(&self, author_id: uuid::Uuid) -> anyhow::Result<Option<Author>>;
}
