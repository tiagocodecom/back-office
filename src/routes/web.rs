use actix_web::web::{get, scope, ServiceConfig};
use actix_web::HttpResponse;

pub fn web_routes(cfg: &mut ServiceConfig) {
    cfg.service(scope("").route("/health-check", get().to(HttpResponse::Ok)));
}
