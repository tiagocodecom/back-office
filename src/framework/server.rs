use crate::framework;
use crate::framework::container::Container;
use actix_web::dev::Server;
use actix_web::web::{get, scope, Data};
use actix_web::{App, HttpResponse, HttpServer};
use std::net::TcpListener;

pub async fn run(listener: TcpListener, container: Container) -> anyhow::Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(container.clone()))
            .route("/", get().to(HttpResponse::Ok))
            .service(scope("/api").configure(framework::routes::api_routes))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
