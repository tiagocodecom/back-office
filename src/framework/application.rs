use crate::framework::routes;
use crate::framework::settings::entities::config::Config;
use crate::Container;
use actix_files::Files;
use actix_web::{
    dev::Server,
    web::{scope, Data},
    App, HttpServer,
};
use std::net::TcpListener;

/// The `Application` struct encapsulates the web server's configuration and runtime.
///
/// This struct is responsible for setting up and running the Actix web server,
/// handling the initialization of routes, static files, and other server dependencies.
pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    /// Creates a new `Application` instance, initializing the server with the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the configuration settings used to initialize the server.
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - Returns an `Ok(Self)` if initialization is successful, or an `Err` if an error occurs.
    ///
    /// # Errors
    ///
    /// This function will return an error if the server fails to bind to the specified address.
    pub async fn new(config: &Config) -> Result<Self, String> {
        let address = format!("{}:{}", &config.application.host, &config.application.port);
        let listener = TcpListener::bind(address.clone()).map_err(|e| e.to_string())?;
        let port = listener.local_addr().map_err(|e| e.to_string())?.port();
        let server = initialize_application(config, listener).await;

        Ok(Self { port, server })
    }

    /// Runs the server, blocking the current thread until the server is stopped.
    ///
    /// This method consumes the `Application` instance and starts the Actix web server,
    /// keeping it running until a stop signal is received.
    ///
    /// # Returns
    ///
    /// * `std::io::Result<()>` - Returns `Ok(())` if the server runs successfully, or an `Err` if an I/O error occurs.

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    /// Retrieves the port number that the server is bound to.
    ///
    /// # Returns
    ///
    /// * `u16` - The port number on which the server is listening.
    pub fn port(&self) -> u16 {
        self.port
    }
}

/// Initializes and configures the Actix web server with the provided settings and listener.
///
/// This function sets up the application routes, services, and dependencies,
/// and then binds the server to the provided `TcpListener`.
///
/// # Arguments
///
/// * `config` - A reference to the configuration settings used to set up the server.
/// * `listener` - A `TcpListener` that the server will use to accept incoming connections.
///
/// # Returns
///
/// * `Result<Server>` - Returns the configured `Server` instance on success, or an `Err` if an error occurs.
///
/// # Errors
///
/// This function will return an error if the server fails to bind the listener.

pub async fn initialize_application(config: &Config, listener: TcpListener) -> Server {
    let container = Container::new(config);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(container.clone()))
            .service(Files::new("/static", "static").show_files_listing())
            .service(scope("/api").configure(routes::api_routes))
            .service(scope("/admin").configure(routes::web_routes))
    })
    .listen(listener)
    .unwrap()
    .run()
}
