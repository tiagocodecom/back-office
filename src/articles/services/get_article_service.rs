use crate::articles::domain::RenderOutput;
use crate::articles::ports::driven::{FetchArticlePort, RenderArticlePort};
use uuid::Uuid;

pub struct GetArticleService<R, P> {
    repository: R,
    presenter: P,
}

impl<R, P> GetArticleService<R, P>
where
    R: FetchArticlePort,
    P: RenderArticlePort<Output = RenderOutput>,
{
    pub fn new(repository: R, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    pub async fn execute(&self, article_id: Uuid) -> anyhow::Result<RenderOutput> {
        let article = self.repository.get_article_by_id(article_id).await?;
        let result = self.presenter.render_article(article)?;

        Ok(result)
    }
}
