use crate::configuration::entities::config::Config;
use crate::container::Container;
use crate::routes;

use crate::shared::database::get_connection_pool;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::HttpServer;
use anyhow::Context;
use sqlx::PgPool;
use std::net::TcpListener;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: Config) -> anyhow::Result<Self> {
        let address = format!("{}:{}", &config.application.host, &config.application.port);
        let listener = TcpListener::bind(address).context("Failed to bind the TCP listener")?;
        let port = listener.local_addr()?.port();
        let db_pool = get_connection_pool(&config.database);

        let server = run(listener, db_pool)
            .await
            .context("Failed to run the server")?;

        Ok(Self { port, server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(listener: TcpListener, db_pool: PgPool) -> anyhow::Result<Server> {
    let server = HttpServer::new(move || {
        actix_web::App::new()
            .app_data(Data::new(db_pool.clone()))
            .app_data(Data::new(Container::load()))
            .configure(routes::api_routes)
            .configure(routes::web_routes)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
