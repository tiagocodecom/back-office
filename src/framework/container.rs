use crate::articles::adapters::driven::HtmlGetArticlePresenter;
use crate::articles::adapters::driven::JsonGetArticlePresenter;
use crate::articles::adapters::driven::PostgresArticleRepository;
use crate::authentication::get_login::{GetLoginHtmlPresenter, GetLoginProvider};
use crate::framework::database::Database;
use crate::framework::handlebars::init_handlebars_engine;
use crate::framework::settings::entities::config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct Container<'a> {
    pub postgres_article_repository: PostgresArticleRepository,
    pub html_get_article_presenter: HtmlGetArticlePresenter<'a>,
    pub json_get_article_presenter: JsonGetArticlePresenter,
    pub login_provider: GetLoginProvider,
    pub login_html_presenter: GetLoginHtmlPresenter<'a>,
}

impl<'a> Container<'a> {
    pub fn new(config: &Config) -> Self {
        let handlebars = Arc::new(init_handlebars_engine().unwrap());
        let connection_pool = Arc::new(Database::new_pool(&config.database));

        Self {
            postgres_article_repository: PostgresArticleRepository::new(connection_pool),
            html_get_article_presenter: HtmlGetArticlePresenter::new(handlebars.clone()),
            json_get_article_presenter: JsonGetArticlePresenter::new(),
            login_provider: GetLoginProvider::new(),
            login_html_presenter: GetLoginHtmlPresenter::new(handlebars.clone()),
        }
    }
}
