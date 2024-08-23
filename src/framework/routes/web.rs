use crate::articles::adapters::driving::web_get_article_controller as web_get_article;
use crate::authentication::adapters::driving::web_get_form_controller as web_get_form;
use actix_web::web::{get, scope, ServiceConfig};
use actix_web::HttpResponse;

pub fn web_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("")
            .route("/health-check", get().to(HttpResponse::Ok))
            .route("/articles/{id}", get().to(web_get_article::handle))
            .route("/auth/{form_id}", get().to(web_get_form::handle)),
    );
}
