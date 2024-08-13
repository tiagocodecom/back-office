use crate::framework::configuration::entities::database::DatabaseConfig;
use secrecy::ExposeSecret;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use sqlx::PgPool;
use std::fmt::Debug;

/// A struct representing the database connection.
///
/// The `Database` struct is responsible for creating a connection pool to the database.
#[derive(Clone)]
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
    pub fn options_without_db_name(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.config.host())
            .username(&self.config.username())
            .password(&self.config.password().expose_secret())
            .port(self.config.port())
            .ssl_mode(self.ssl_mode())
    }

    pub fn options_with_db_name(&self) -> PgConnectOptions {
        self.options_without_db_name()
            .database(&self.config.db_name())
    }

    fn connection_pool(&self) -> PgPool {
        PgPoolOptions::new().connect_lazy_with(self.options_with_db_name())
    }
}

impl Debug for Database<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database")
            .field("host", &self.config.host())
            .field("port", &self.config.port())
            .field("username", &self.config.username())
            .field("password", &self.config.password())
            .field("db_name", &self.config.db_name())
            .finish()
    }
}
