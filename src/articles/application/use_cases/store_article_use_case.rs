use crate::articles::application::ports::driven::GetArticleRepository;
use crate::articles::application::ports::driven::{GetArticlePresenter, StoreArticleRepository};
use crate::articles::application::ports::driving::StoreArticleService;
use crate::articles::entities::{Error, NewArticle, RenderOutput};
use async_trait::async_trait;

pub struct StoreArticleUseCase<R: StoreArticleRepository, P: GetArticlePresenter> {
    repository: R,
    presenter: P,
}

impl<R, P> StoreArticleUseCase<R, P>
where
    R: StoreArticleRepository + GetArticleRepository,
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
impl<R, P> StoreArticleService for StoreArticleUseCase<R, P>
where
    R: StoreArticleRepository + GetArticleRepository,
    P: GetArticlePresenter<Output = RenderOutput>,
{
    type Output = RenderOutput;

    async fn execute(&self, new_article: &NewArticle) -> Result<Self::Output, Error> {
        // Ideally we would check if the user is authorized to create an article here.
        // As we don't have authentication in this example, we'll just create the article.

        let article_id = self.repository.store_article(new_article).await?;
        let article = self.repository.get_article_by_id(&article_id).await?;

        // Ideally we would check if the user has permission to view the article here

        self.presenter.present_article(article)
    }
}
