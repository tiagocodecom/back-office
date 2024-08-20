use crate::articles::application::domain::Article;
use crate::articles::application::ports::driven::FetchArticlePort;
use crate::articles::application::ports::driving::GetArticleUseCase;
use async_trait::async_trait;
use uuid::Uuid;

pub struct GetArticleService<R: FetchArticlePort> {
    repository: R,
}

impl<R: FetchArticlePort> GetArticleService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn with_repository(repository: R, article_id: &Uuid) -> anyhow::Result<Article> {
        Self::new(repository).execute(article_id).await
    }
}

#[async_trait(?Send)]
impl<R: FetchArticlePort> GetArticleUseCase for GetArticleService<R> {
    async fn execute(&self, article_id: &Uuid) -> anyhow::Result<Article> {
        self.repository.get_by_id(article_id).await
    }
}
