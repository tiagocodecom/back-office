use crate::configuration::entities::config::Config;
use crate::container::Container;
use crate::routes;
use crate::startup::database::get_connection_pool;
use actix_web::dev::Server;
use actix_web::web::{get, scope, Data};
use actix_web::{App, HttpResponse, HttpServer};
use anyhow::Context;
use sqlx::PgPool;
use std::net::TcpListener;

/// A struct representing the main application.
///
/// The `Application` struct is responsible for setting up and running the Actix web server.
/// It holds the server instance and the port on which the server is running.
///
/// # Fields
///
/// * `port` - The port number on which the server is listening.
/// * `server` - The Actix web server instance.
pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    /// Initializes the `Application` by configuring the server and its dependencies.
    /// Returns an `Application` instance if the server is successfully configured.
    pub async fn build(config: Config) -> anyhow::Result<Self> {
        let address = format!("{}:{}", &config.application.host, &config.application.port);
        let listener = TcpListener::bind(address.clone())?;
        let port = listener.local_addr()?.port();

        let db_pool = get_connection_pool(&config.database);
        let container = Container::new(config);

        let server = run(listener, db_pool, container)
            .await
            .context("Failed to run the server")?;

        dbg!(
            "Server is running on address {}, port: {}",
            address,
            port.clone()
        );

        Ok(Self { port, server })
    }

    /// Starts the server and blocks the current thread until the server is stopped.
    /// This method runs asynchronously and will only return when the server is halted.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    container: Container,
) -> anyhow::Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .app_data(Data::new(container.clone()))
            .route("/health-check", get().to(HttpResponse::Ok))
            .service(scope("/api").configure(routes::api_routes))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
