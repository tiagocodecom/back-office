use crate::framework::configuration::entities::application::ApplicationConfig;
use crate::framework::configuration::entities::database::DatabaseConfig;

#[derive(serde::Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub application: ApplicationConfig,
}
