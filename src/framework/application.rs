use crate::framework::configuration::entities::config::Config;
use crate::framework::container::Container;
use crate::framework::server::run;
use actix_web::dev::Server;
use std::net::TcpListener;

/// A struct representing the main application.
///
/// The `Application` struct is responsible for setting up and running the Actix web server.
/// It holds the server instance and the port on which the server is running.
pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    /// Initializes the `Application` by configuring the server and its dependencies.
    /// Returns an `Application` instance if the server is successfully configured.
    pub async fn from(config: &Config) -> anyhow::Result<Self> {
        let address = format!("{}:{}", &config.application.host, &config.application.port);
        let listener = TcpListener::bind(address.clone())?;
        let port = listener.local_addr()?.port();

        let container = Container::new(config);
        let server = run(listener, container).await?;

        Ok(Self { port, server })
    }

    /// Starts the server and blocks the current thread until the server is stopped.
    /// This method runs asynchronously and will only return when the server is halted.
    pub async fn run_server(self) -> Result<(), std::io::Error> {
        println!("Server running on port: {}", self.port().clone());
        self.server().await
    }

    /// Returns the port on which the server is running.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns a reference to the server instance.
    pub fn server(self) -> Server {
        self.server
    }
}
