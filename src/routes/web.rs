use crate::articles::adapters as articles_adapters;
use actix_web::web::{self, post};

pub fn web_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/web").route("/articles", post().to(articles_adapters::web::save_article)),
    );
}
