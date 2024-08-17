use crate::articles::application::ports::primary::GetArticleUseCase;
use crate::articles::application::ports::secondary::ForGettingArticle;
use crate::articles::domain::Article;
use async_trait::async_trait;
use uuid::Uuid;

pub struct GetArticleService<R: ForGettingArticle> {
    repository: R,
}

impl<R: ForGettingArticle> GetArticleService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute_with_repository(
        repository: R,
        article_id: &Uuid,
    ) -> anyhow::Result<Article> {
        Self::new(repository).execute(article_id).await
    }
}

#[async_trait(?Send)]
impl<R: ForGettingArticle> GetArticleUseCase for GetArticleService<R> {
    async fn execute(&self, article_id: &Uuid) -> anyhow::Result<Article> {
        self.repository.get_by_id(article_id).await
    }
}
