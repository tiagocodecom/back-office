use crate::articles::adapters::driven::ArticleRepository;
use crate::framework::database::Database;
use crate::framework::handlebars::init_handlebars_engine;
use crate::framework::settings::entities::config::Config;
use handlebars::Handlebars;
use std::sync::Arc;

#[derive(Clone)]
pub struct Container {
    pub article_repository: ArticleRepository,
    pub view: Arc<Handlebars<'static>>,
}

impl Container {
    pub fn new(config: &Config) -> Self {
        let connection_pool = Arc::new(Database::new_pool(&config.database));
        let handlebars = Arc::new(init_handlebars_engine().unwrap());

        Self {
            view: handlebars,
            article_repository: ArticleRepository::new(connection_pool.clone()),
        }
    }
}
