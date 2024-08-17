use crate::articles::adapters::primary::web as articles_web;
use actix_web::web::{get, scope, ServiceConfig};
use actix_web::HttpResponse;

pub fn web_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("")
            .route("/articles/{id}", get().to(articles_web::show_article))
            .route("/health-check", get().to(HttpResponse::Ok)),
    );
}
