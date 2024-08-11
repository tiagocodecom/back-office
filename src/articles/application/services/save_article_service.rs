use crate::articles::application::ports::ForSavingArticle;
use crate::articles::domain::{Article, NewArticle};
use sqlx::PgPool;

pub async fn execute(
    db_pool: &PgPool,
    driven_port: &impl ForSavingArticle,
    article: &NewArticle,
) -> anyhow::Result<Article>
where
{
    driven_port.save_article(db_pool, article).await
}
