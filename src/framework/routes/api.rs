use crate::articles::adapters as articles_adapters;
use actix_web::web::{get, post, resource, ServiceConfig};
use actix_web::HttpResponse;

pub fn api_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/articles")
            .route(post().to(articles_adapters::web::save_article))
            .route(get().to(HttpResponse::Ok)),
    );
}
