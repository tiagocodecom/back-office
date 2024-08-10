use crate::articles::adapters::article_controller;
use actix_web::web::{self, get};

pub fn web_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/articles", get().to(article_controller::get_articles))
            .route("/articles/{id}", get().to(article_controller::get_articles)),
    );
}
