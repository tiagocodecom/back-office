use crate::configuration::entities::database::DatabaseConfig;
use secrecy::ExposeSecret;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};

pub fn get_connection_pool(config: &DatabaseConfig) -> sqlx::PgPool {
    let ssl_mode = if config.require_ssl() {
        PgSslMode::Require
    } else {
        PgSslMode::Prefer
    };

    PgPoolOptions::new().connect_lazy_with(
        PgConnectOptions::new()
            .host(&config.host())
            .username(&config.username())
            .password(&config.password().expose_secret())
            .port(config.port())
            .ssl_mode(ssl_mode),
    )
}
