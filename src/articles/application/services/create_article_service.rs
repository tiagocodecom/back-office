use crate::articles::application::domain::NewArticle;
use crate::articles::application::ports::driven::StoreArticlePort;
use crate::articles::application::ports::driving::CreateArticleUseCase;
use async_trait::async_trait;
use uuid::Uuid;

pub struct CreateArticleService<R: StoreArticlePort> {
    repository: R,
}

impl<R: StoreArticlePort> CreateArticleService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn with_repository(repository: R, article: &NewArticle) -> anyhow::Result<Uuid> {
        Self::new(repository).execute(article).await
    }
}

#[async_trait(?Send)]
impl<R: StoreArticlePort> CreateArticleUseCase for CreateArticleService<R> {
    async fn execute(&self, article: &NewArticle) -> anyhow::Result<Uuid> {
        self.repository.create(article).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::articles::application::domain::NewArticle;
    use crate::articles::application::ports::driven::MockStoreArticlePort;
    use mockall::predicate::*;

    #[tokio::test]
    async fn create_article_returns_expected_uuid() {
        let mut repository_mock = MockStoreArticlePort::new();
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
        let mut mock = MockStoreArticlePort::new();
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
