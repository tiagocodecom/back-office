use crate::articles::application::ports::SaveArticleUseCase;
use crate::articles::domain::{Article, NewArticle};

pub async fn execute<T>(driven_port: &T, article: &NewArticle) -> anyhow::Result<Article>
where
    T: SaveArticleUseCase,
{
    driven_port.save_article(article).await
}
