use crate::framework::settings::entities::application::ApplicationConfig;
use crate::framework::settings::entities::database::DatabaseConfig;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub database: DatabaseConfig,
    pub application: ApplicationConfig,
}
