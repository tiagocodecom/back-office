use crate::articles::application::ports::primary::CreateArticleUseCase;
use crate::articles::application::ports::secondary::ForCreatingArticle;
use crate::articles::domain::NewArticle;
use async_trait::async_trait;
use uuid::Uuid;

pub struct CreateArticleService<R: ForCreatingArticle> {
    repository: R,
}

impl<R: ForCreatingArticle> CreateArticleService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute_with_repository(
        repository: R,
        article: &NewArticle,
    ) -> anyhow::Result<Uuid> {
        Self::new(repository).execute(article).await
    }
}

#[async_trait(?Send)]
impl<R: ForCreatingArticle> CreateArticleUseCase for CreateArticleService<R> {
    async fn execute(&self, article: &NewArticle) -> anyhow::Result<Uuid> {
        self.repository.create(article).await
    }
}
