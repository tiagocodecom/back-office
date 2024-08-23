use crate::articles::get_article::{GetArticleHtmlPresenter, GetArticleJsonPresenter};
use crate::articles::PostgresArticleRepository;
use crate::authentication::get_login::{GetLoginHtmlPresenter, GetLoginProvider};
use crate::framework::database::Database;
use crate::framework::handlebars::init_handlebars_engine;
use crate::framework::settings::entities::config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct Container<'a> {
    pub article_postgres_repository: PostgresArticleRepository,
    pub article_html_presenter: GetArticleHtmlPresenter<'a>,
    pub article_json_presenter: GetArticleJsonPresenter,
    pub login_provider: GetLoginProvider,
    pub login_html_presenter: GetLoginHtmlPresenter<'a>,
}

impl<'a> Container<'a> {
    pub fn new(config: &Config) -> Self {
        let handlebars = Arc::new(init_handlebars_engine().unwrap());
        let connection_pool = Arc::new(Database::new_pool(&config.database));

        Self {
            article_postgres_repository: PostgresArticleRepository::new(connection_pool),
            article_html_presenter: GetArticleHtmlPresenter::new(handlebars.clone()),
            article_json_presenter: GetArticleJsonPresenter::new(),
            login_provider: GetLoginProvider::new(),
            login_html_presenter: GetLoginHtmlPresenter::new(handlebars.clone()),
        }
    }
}
