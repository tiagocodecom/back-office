use crate::articles::create_article::create_article_api_controller as api_create_article;
use crate::articles::get_article::get_article_api_controller as api_get_article;
use actix_web::web::{get, post, scope, ServiceConfig};

pub fn api_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/articles")
            .route("", post().to(api_create_article::handle))
            .route("/{id}", get().to(api_get_article::handle)),
    );
}
