use crate::articles::application::ports::driven::GetArticlePresenter;
use crate::articles::application::ports::driven::GetArticleRepository;
use crate::articles::application::ports::driving::GetArticleService;
use crate::articles::entities::{Error, RenderOutput};
use async_trait::async_trait;
use uuid::Uuid;

pub struct GetArticleUseCase<R, P> {
    repository: R,
    presenter: P,
}

impl<R, P> GetArticleUseCase<R, P>
where
    R: GetArticleRepository,
    P: GetArticlePresenter,
{
    pub fn new(repository: R, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }
}

#[async_trait(?Send)]
impl<R, P> GetArticleService for GetArticleUseCase<R, P>
where
    R: GetArticleRepository,
    P: GetArticlePresenter<Output = RenderOutput>,
{
    type Output = RenderOutput;

    async fn execute(&self, article_id: &Uuid) -> Result<Self::Output, Error> {
        // Ideally we would check if the user has permission to view the article here

        let article = self.repository.get_article_by_id(article_id).await?;

        // Ideally we would check if the user has permission to view the article here

        self.presenter.present_article(article)
    }
}

#[cfg(test)]
mod tests {

    // fn sample_article() -> Article {
    //     Article::new(
    //         Uuid::new_v4(),
    //         Uuid::new_v4(),
    //         sentence(5),
    //         paragraph(5, 5, 10, "".into()),
    //         paragraph(5, 5, 10, "".into()),
    //     )
    // }
    //
    // #[tokio::test]
    // async fn returns_an_existing_article_rendered_as_json() {
    //     let mut repository = MockGetArticleRepository::new();
    //     let mut presenter = MockGetArticlePresenter::new();
    //
    //     let article = sample_article();
    //     let article_id = article.id().clone();
    //     let repository_response = article.clone();
    //
    //     repository
    //         .expect_get_article_by_id()
    //         .with(eq(article_id))
    //         .times(1)
    //         .return_once_st(move |_| Ok(repository_response));
    //
    //     presenter
    //         .expect_present_article()
    //         .with(eq(article.clone()))
    //         .times(1)
    //         .return_once(|article| Ok(RenderOutput::Json(serde_json::json!(article))));
    //
    //     let get_article_service = GetArticleUseCase::new(repository, presenter);
    //     let result = get_article_service.execute(&article_id).await;
    //
    //     assert_ok!(&result);
    //     assert_eq!(
    //         result.unwrap(),
    //         RenderOutput::Json(serde_json::json!(article))
    //     );
    // }
}
