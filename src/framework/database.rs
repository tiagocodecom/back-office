use crate::framework::configuration::entities::database::DatabaseConfig;
use secrecy::ExposeSecret;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use sqlx::PgPool;

/// A struct representing the database connection.
///
/// The `Database` struct is responsible for creating a connection pool to the database.
pub struct Database<'a> {
    config: &'a DatabaseConfig,
}

impl<'a> Database<'a> {
    /// Creates a new `Database` instance from the provided `DatabaseConfig`.
    pub fn from(config: &'a DatabaseConfig) -> Self {
        Self { config }
    }

    /// Creates a new connection pool to the database from the provided `DatabaseConfig`
    pub fn new_pool(cfg: &'a DatabaseConfig) -> PgPool {
        Self::from(cfg).connection_pool()
    }

    /// Returns the SSL mode for the database connection.
    fn ssl_mode(&self) -> PgSslMode {
        if self.config.require_ssl() {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        }
    }

    /// Creates a new connection pool to the database.
    fn connection_pool(&self) -> PgPool {
        PgPoolOptions::new().connect_lazy_with(
            PgConnectOptions::new()
                .host(&self.config.host())
                .username(&self.config.username())
                .password(&self.config.password().expose_secret())
                .port(self.config.port())
                .ssl_mode(self.ssl_mode()),
        )
    }
}
