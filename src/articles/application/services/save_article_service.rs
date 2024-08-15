use crate::articles::application::ports::SaveArticleUseCase;
use crate::articles::domain::NewArticle;
use uuid::Uuid;

pub async fn execute<T>(driven_port: &T, article: &NewArticle) -> anyhow::Result<Uuid>
where
    T: SaveArticleUseCase,
{
    driven_port.save_article(article).await
}
