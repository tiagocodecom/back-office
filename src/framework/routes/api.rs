use crate::articles::adapters::driving::api_create_article_controller as api_create_article;
use crate::articles::adapters::driving::api_get_article_controller as api_get_article;
use actix_web::web::{get, post, scope, ServiceConfig};

pub fn api_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/articles")
            .route("", post().to(api_create_article::handle))
            .route("/{id}", get().to(api_get_article::handle)),
    );
}
