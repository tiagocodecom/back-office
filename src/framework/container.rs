use crate::articles::adapters::persistence::ArticleRepository;
use crate::framework::configuration::entities::config::Config;
use crate::framework::database::Database;
use std::sync::Arc;

#[derive(Clone)]
pub struct Container {
    pub article_repository: ArticleRepository,
}

impl Container {
    pub fn new(config: &Config) -> Self {
        let connection_pool = Arc::new(Database::new_pool(&config.database));

        Self {
            article_repository: ArticleRepository::new(connection_pool.clone()),
        }
    }
}
