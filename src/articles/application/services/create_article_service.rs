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

    pub async fn with_repository(repository: R, article: &NewArticle) -> anyhow::Result<Uuid> {
        Self::new(repository).execute(article).await
    }
}

#[async_trait(?Send)]
impl<R: ForCreatingArticle> CreateArticleUseCase for CreateArticleService<R> {
    async fn execute(&self, article: &NewArticle) -> anyhow::Result<Uuid> {
        self.repository.create(article).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::articles::application::ports::secondary::MockForCreatingArticle;
    use crate::articles::domain::NewArticle;
    use mockall::predicate::*;

    #[tokio::test]
    async fn create_article_returns_expected_uuid() {
        let mut repository_mock = MockForCreatingArticle::new();
        let article_id = Uuid::new_v4();
        let new_article = NewArticle::new(
            Uuid::new_v4(),
            String::from("What is Lorem Ipsum?"),
            String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit."),
        );

        repository_mock
            .expect_create()
            .with(eq(new_article.clone()))
            .times(1)
            .returning(move |_| Ok(article_id.clone()));

        let result = CreateArticleService::with_repository(repository_mock, &new_article).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), article_id);
    }

    #[tokio::test]
    async fn create_article_returns_error_failure() {
        let mut mock = MockForCreatingArticle::new();
        let new_article = NewArticle::new(
            Uuid::new_v4(),
            String::from("What is Lorem Ipsum?"),
            String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit."),
        );

        mock.expect_create()
            .with(eq(new_article.clone()))
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Failed to create article")));

        let result = mock.create(&new_article).await;

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Failed to create article"
        );
    }
}
