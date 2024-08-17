use crate::framework::container::Container;
use crate::framework::routes;
use crate::framework::settings::entities::config::Config;
use actix_files::Files;
use actix_web::dev::Server;
use actix_web::web::{scope, Data};
use actix_web::{App, HttpServer};
use std::net::TcpListener;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    /// Initializes the `Application` by configuring the server and its dependencies.
    pub async fn from(config: &Config) -> anyhow::Result<Self> {
        let address = format!("{}:{}", &config.application.host, &config.application.port);
        let listener = TcpListener::bind(address.clone())?;
        let port = listener.local_addr()?.port();

        let container = Container::new(config);

        let server = HttpServer::new(move || {
            App::new()
                .app_data(Data::new(container.clone()))
                .service(Files::new("/static", "static").show_files_listing())
                .service(scope("/api").configure(routes::api_routes))
                .service(scope("/admin").configure(routes::web_routes))
        })
        .listen(listener)?
        .run();

        Ok(Self { port, server })
    }

    /// Starts the server and blocks the current thread until the server is stopped.
    pub async fn run_server(self) -> Result<(), std::io::Error> {
        self.server().await
    }

    /// Returns the port the server is running on.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns the server instance.
    pub fn server(self) -> Server {
        self.server
    }
}
