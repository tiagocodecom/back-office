use std::path::PathBuf;

pub trait SettingsFileFilter {
    /// Determines if the given path is a configuration file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file.
    ///
    fn is_config_file(&self, path: &PathBuf) -> bool;

    /// Filters the list of paths to only include configuration files.
    ///
    /// # Arguments
    ///
    /// * `entries` - The list of paths to filter.
    ///
    fn filter_config_files(&self, entries: Vec<PathBuf>) -> Vec<PathBuf> {
        entries
            .into_iter()
            .filter(|path| self.is_config_file(path))
            .collect()
    }
}

pub struct TomlSettingsFileFilter;

impl TomlSettingsFileFilter {
    pub fn new() -> Self {
        Self
    }
}

impl SettingsFileFilter for TomlSettingsFileFilter {
    fn is_config_file(&self, path: &PathBuf) -> bool {
        path.file_name()
            .and_then(|name| name.to_str())
            .map_or(false, |name| name.contains(".config.toml"))
    }
}
