use crate::articles::adapters::primary::api as articles_api;
use actix_web::web::{get, post, scope, ServiceConfig};

pub fn api_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/articles")
            .route("", post().to(articles_api::save_article))
            .route("/{id}", get().to(articles_api::get_article)),
    );
}
