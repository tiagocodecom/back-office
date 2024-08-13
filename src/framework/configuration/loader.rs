use crate::framework::configuration::entities::config::Config;
use config::Config as ConfigRepository;
use config::File;
use std::env;
use std::path::PathBuf;

/// A struct responsible for loading configuration files from a specified directory.
///
/// The `ConfigLoader` manages the path to the configuration directory and provides methods
/// to load configuration data from YAML files.
pub struct ConfigLoader {
    directory: String,
}

impl ConfigLoader {
    /// Creates a `ConfigLoader` with the given directory path.
    ///
    /// # Arguments
    /// * `directory` - Path to the configuration directory (from the project root).
    pub fn new(directory: String) -> Self {
        Self { directory }
    }

    /// Creates a `ConfigLoader` with a custom directory path, and
    /// deserializes the configuration data.
    ///
    /// # Arguments
    /// * `directory` - Path to the configuration directory (from the project root).
    pub fn from_custom_dir(directory: String) -> anyhow::Result<Config> {
        Self::new(directory).load()
    }

    /// Creates a `ConfigLoader` from the default directory `config` located
    /// at the root of the project, and deserializes the configuration data.
    pub fn from_default_dir() -> anyhow::Result<Config> {
        Self::new("config".to_string()).load()
    }

    /// Resolves the full path to the configuration directory.
    /// Returns an error if the directory does not exist or current directory cannot be determined.
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

    /// Loads configuration from YAML files in the specified directory.
    /// Reads `application.yaml` and `database.yaml`, deserializing them into a `Config` object.
    fn load(&self) -> anyhow::Result<Config> {
        let config_directory = self.get_config_directory()?;
        let config_data = ConfigRepository::builder()
            .add_source(File::from(config_directory.join("application.yaml")))
            .add_source(File::from(config_directory.join("database.yaml")))
            .build()?;

        config_data
            .try_deserialize::<Config>()
            .map_err(|e| anyhow::anyhow!("Failed to deserialize the configuration: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_error_when_config_directory_does_not_exists() {
        let config_loader = ConfigLoader::from_custom_dir("non-existing-dir".to_string());
        assert!(config_loader.is_err());
    }

    #[test]
    fn when_using_from_default_the_config_directory_must_exists_at_project_root_level() {
        let config_loader = ConfigLoader::from_default_dir();
        assert!(config_loader.is_ok());
    }

    #[test]
    fn should_load_and_deserialize_configuration_into_entity_structs() {
        let config_loader = ConfigLoader::from_custom_dir("tests/fixtures/config".to_string());
        let config = config_loader.unwrap();

        assert_eq!(config.application.name, "test_application");
        assert_eq!(config.application.version, "1.0.0");
        assert_eq!(config.application.environment, "testing");
    }
}
