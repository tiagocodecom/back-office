use crate::articles::adapters as articles_adapters;
use actix_web::web::{self, get, post};

pub fn web_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/articles", get().to(articles_adapters::api::get_articles))
            .route("/articles", post().to(articles_adapters::api::save_article)),
    );
}
