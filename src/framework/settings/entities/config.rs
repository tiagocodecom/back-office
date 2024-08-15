use crate::framework::settings::entities::application::ApplicationConfig;
use crate::framework::settings::entities::database::DatabaseConfig;

#[derive(serde::Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub application: ApplicationConfig,
}
