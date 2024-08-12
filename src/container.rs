use crate::articles::adapters::persistence::ArticleRepository;
use crate::configuration::entities::config::Config;
use crate::startup::database::get_connection_pool;
use std::sync::Arc;

#[derive(Clone)]
pub struct Container {
    pub article_repository: ArticleRepository,
}

impl Container {
    pub fn new(config: Config) -> Self {
        let db_pool = get_connection_pool(&config.database);

        Self {
            article_repository: ArticleRepository::new(Arc::new(db_pool)),
        }
    }
}
