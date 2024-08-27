use crate::articles::get_article::{GetArticleError, GetArticlePort, RenderArticlePort};
use crate::articles::RenderOutput;
use uuid::Uuid;

pub struct GetArticleService<R, P> {
    repository: R,
    presenter: P,
}

impl<R, P> GetArticleService<R, P>
where
    R: GetArticlePort,
    P: RenderArticlePort<Output = RenderOutput>,
{
    pub fn new(repository: R, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    pub async fn execute(&self, article_id: Uuid) -> Result<RenderOutput, GetArticleError> {
        let article = self.repository.get_article_by_id(article_id).await?;
        let result = self.presenter.render_article(article)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::articles::get_article::get_article_error::GetArticleError;
    use crate::articles::get_article::*;
    use crate::articles::Article;
    use claim::*;
    use fakeit::words::{paragraph, sentence};
    use mockall::predicate::*;

    fn sample_article() -> Article {
        Article::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            sentence(5),
            paragraph(5, 5, 10, "".into()),
            paragraph(5, 5, 10, "".into()),
        )
    }

    #[tokio::test]
    async fn returns_raw_render_output_for_existing_article() {
        let mut repository = MockGetArticlePort::new();
        let mut presenter = MockRenderArticlePort::new();

        let article = sample_article();
        let article_id = article.id().clone();
        let repository_response = article.clone();

        repository
            .expect_get_article_by_id()
            .with(eq(article_id))
            .times(1)
            .return_once_st(move |_| Ok(repository_response));

        presenter
            .expect_render_article()
            .with(eq(article.clone()))
            .times(1)
            .return_once(|article| Ok(RenderOutput::Raw(article)));

        let get_article_service = GetArticleService::new(repository, presenter);
        let result = get_article_service.execute(article_id).await;

        assert_ok!(&result);
        assert_eq!(result.unwrap(), RenderOutput::Raw(article));
    }

    #[tokio::test]
    async fn returns_error_when_article_does_not_exists() {
        let mut repository = MockGetArticlePort::new();
        let mut presenter = MockRenderArticlePort::new();

        let article_id = Uuid::new_v4();

        repository
            .expect_get_article_by_id()
            .with(eq(article_id))
            .times(1)
            .return_once(|article_id| Err(GetArticleError::NotFound(article_id.to_string())));

        presenter.expect_render_article().times(0);

        let get_article_service = GetArticleService::new(repository, presenter);
        let result = get_article_service.execute(article_id.clone()).await;

        assert_err!(&result);
        assert_eq!(
            result,
            Err(GetArticleError::NotFound(article_id.to_string()))
        );
    }

    #[tokio::test]
    async fn returns_error_when_article_exists_but_rendering_fails() {
        let mut repository = MockGetArticlePort::new();
        let mut presenter = MockRenderArticlePort::new();

        let article = sample_article();
        let article_id = article.id().clone();
        let repository_response = article.clone();

        repository
            .expect_get_article_by_id()
            .with(eq(article_id))
            .times(1)
            .return_once_st(move |_| Ok(repository_response));

        presenter
            .expect_render_article()
            .with(eq(article.clone()))
            .times(1)
            .return_once(|_| Err(GetArticleError::Presenter("".into())));

        let get_article_service = GetArticleService::new(repository, presenter);
        let result = get_article_service.execute(article_id).await;

        assert_err!(&result);
        assert_eq!(result, Err(GetArticleError::Presenter("".into())));
    }
}
