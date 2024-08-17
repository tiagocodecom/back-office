use crate::articles::adapters::primary::api as articles_web_adapters;
use actix_web::web::{get, post, scope, ServiceConfig};

pub fn api_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/articles")
            .route("", post().to(articles_web_adapters::save_article))
            .route("/{id}", get().to(articles_web_adapters::get_article)),
    );
}
