use crate::articles::application::ports::ForGettingAuthor;
use crate::articles::domain::Author;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthorRepository {}

#[async_trait(?Send)]
impl ForGettingAuthor for AuthorRepository {
    async fn find_author(&self, author_id: Uuid) -> anyhow::Result<Option<Author>> {
        todo!()
    }
}
