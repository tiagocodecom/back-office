use crate::configuration::entities::config::Config;
use config::Config as ConfigRepository;
use config::File;
use std::env;
use std::path::PathBuf;

///
///
pub struct ConfigLoader {
    directory: String,
}

impl ConfigLoader {
    pub fn from_dir() -> Self {
        let directory = String::from("config");
        Self { directory }
    }

    fn get_config_directory(&self) -> anyhow::Result<PathBuf> {
        let base_directory = env::current_dir()
            .map_err(|_| anyhow::anyhow!("Failed to determine the current directory"))?;
        let config_directory = base_directory.join(&self.directory);

        if config_directory.exists() {
            Ok(config_directory)
        } else {
            Err(anyhow::anyhow!("Failed to determine the config directory"))
        }
    }

    pub fn load(&self) -> anyhow::Result<Config> {
        let config_directory = self.get_config_directory()?;
        let config_data = ConfigRepository::builder()
            .add_source(File::from(config_directory.join("application.yaml")))
            .add_source(File::from(config_directory.join("database.yaml")))
            .build()?;

        config_data
            .try_deserialize::<Config>()
            .map_err(|e| anyhow::anyhow!("Failed deserializing the configuration: {}", e))
    }
}
