use crate::configuration::entities::application::ApplicationConfig;
use crate::configuration::entities::database::DatabaseConfig;

#[derive(serde::Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub application: ApplicationConfig,
}
