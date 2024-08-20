use crate::framework::settings::directory_reader::{DefaultDirectoryReader, DirectoryReader};
use crate::framework::settings::entities::config::Config as ConfigEntity;
use crate::framework::settings::entities::environment::Environment;
use crate::framework::settings::settings_directory::SettingsDirectory;
use crate::framework::settings::settings_file_filter::SettingsFileFilter;
use crate::framework::settings::settings_file_filter::TomlSettingsFileFilter;
use config::{Config, File};
use std::path::PathBuf;

pub enum SettingsOverwrite {
    Enabled,
    Disabled,
}

pub struct SettingsLoader<R: DirectoryReader, F: SettingsFileFilter> {
    reader: R,
    filter: F,
    files: Vec<PathBuf>,
    overwrites: SettingsOverwrite,
    environment: Environment,
    directory: SettingsDirectory,
}

impl Default for SettingsLoader<DefaultDirectoryReader, TomlSettingsFileFilter> {
    fn default() -> Self {
        Self::new(
            SettingsDirectory::new("config".into()).unwrap(),
            SettingsOverwrite::Enabled,
            Environment::default(),
            DefaultDirectoryReader::new(),
            TomlSettingsFileFilter::new(),
        )
    }
}

impl<R: DirectoryReader, F: SettingsFileFilter> SettingsLoader<R, F> {
    pub fn new(
        directory: SettingsDirectory,
        overwrites: SettingsOverwrite,
        environment: Environment,
        reader: R,
        filter: F,
    ) -> Self {
        Self {
            reader,
            filter,
            directory,
            environment,
            overwrites,
            files: vec![],
        }
    }

    pub fn directory(&self) -> &SettingsDirectory {
        &self.directory
    }

    pub fn change_directory(&mut self, directory: &str) -> &mut Self {
        self.directory = SettingsDirectory::new(directory).unwrap();
        self
    }

    fn is_overwrite_enabled(&self) -> bool {
        matches!(self.overwrites, SettingsOverwrite::Enabled)
    }

    pub fn disable_overwrites(&mut self) -> &mut Self {
        self.overwrites = SettingsOverwrite::Disabled;
        self
    }

    pub fn load_files(&mut self) -> &mut Self {
        self.files
            .extend([self.base_files(), self.overwrites_files()].concat());
        self
    }

    fn overwrites_files(&self) -> Vec<PathBuf> {
        if !self.is_overwrite_enabled() {
            return vec![];
        }

        let directory = self.directory.join(&self.environment.as_str());
        let entries = self.reader.read_directory(&directory).unwrap();

        self.filter.filter_config_files(entries)
    }

    fn base_files(&self) -> Vec<PathBuf> {
        let directory = &self.directory().as_ref();
        let entries = self.reader.read_directory(directory).unwrap();

        self.filter.filter_config_files(entries)
    }

    pub fn deserialize(&self) -> anyhow::Result<ConfigEntity> {
        self.files
            .iter()
            .fold(Config::builder(), |config, file| {
                config.add_source(File::from(file.clone()).required(false))
            })
            .build()?
            .try_deserialize::<ConfigEntity>()
            .map_err(|e| anyhow::anyhow!("Failed to deserialize the configuration: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_project_root_config_directory_by_default() {
        let loader = SettingsLoader::default();
        assert!(loader.directory().to_string().contains("config"));
    }

    #[test]
    fn allows_overriding_the_config_directory() {
        let mut loader = SettingsLoader::default();

        assert!(loader
            .change_directory("tests/fixtures/config")
            .directory()
            .to_string()
            .contains("tests/fixtures/config"),);
    }

    #[test]
    #[should_panic]
    fn returns_error_if_config_directory_does_not_exist() {
        let config_directory = SettingsDirectory::new("non-existing-dir");
        assert!(config_directory.is_err());

        let mut config_loader = SettingsLoader::default();
        config_loader.change_directory("non-existing-dir");
    }

    #[test]
    fn deserializes_configuration_files_correctly() {
        let config = SettingsLoader::default()
            .change_directory("tests/fixtures/config")
            .disable_overwrites()
            .load_files()
            .deserialize()
            .unwrap();

        assert_eq!(config.application.environment, "testing");
        assert_eq!(config.application.name, "tests-back-office");
    }

    #[test]
    fn merges_with_dev_config_by_default() {
        std::env::set_var("APP__ENVIRONMENT", "development");

        let config = SettingsLoader::default()
            .change_directory("tests/fixtures/config")
            .load_files()
            .deserialize()
            .unwrap();

        assert_eq!(config.application.name, "dev-overwritten-tests");
        assert_eq!(config.application.environment, "testing");
        assert_eq!(config.application.version, "1.0.0");
    }

    #[test]
    fn merges_with_prod_config_when_env_is_specified() {
        std::env::set_var("APP__ENVIRONMENT", "production");

        let config = SettingsLoader::default()
            .change_directory("tests/fixtures/config")
            .load_files()
            .deserialize()
            .unwrap();

        assert_eq!(config.application.name, "prod-overwritten-tests");
        assert_eq!(config.application.environment, "testing");
        assert_eq!(config.application.version, "1.0.0");
    }
}
