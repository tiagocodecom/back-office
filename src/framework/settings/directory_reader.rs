use std::path::PathBuf;

/// Helper trait for reading directories.
pub trait DirectoryReader {
    /// Reads the directory and returns a list of paths to the files in the directory.
    ///
    /// # Arguments
    ///
    /// * `directory` - The directory to read (relative to the project root).
    ///
    fn read_directory(&self, directory: &PathBuf) -> Result<Vec<PathBuf>, anyhow::Error>;
}

pub struct DefaultDirectoryReader;

impl DefaultDirectoryReader {
    pub fn new() -> Self {
        Self
    }
}

impl DirectoryReader for DefaultDirectoryReader {
    fn read_directory(&self, directory: &PathBuf) -> Result<Vec<PathBuf>, anyhow::Error> {
        Ok(std::fs::read_dir(directory)?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .collect::<Vec<PathBuf>>())
    }
}
