use crate::articles::adapters::driving::web_get_article_controller as web_get_article;
use crate::authentication::adapters::primary::web as authentication_web;
use actix_web::web::{get, scope, ServiceConfig};
use actix_web::HttpResponse;

pub fn web_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("")
            .route("/articles/{id}", get().to(web_get_article::handle))
            .route("/health-check", get().to(HttpResponse::Ok))
            .route("/auth/login", get().to(authentication_web::show_login)),
    );
}
