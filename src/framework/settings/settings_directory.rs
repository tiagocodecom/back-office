use std::fmt::Display;
use std::ops::Deref;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct SettingsDirectory(PathBuf);

impl SettingsDirectory {
    pub fn new(dir: &str) -> anyhow::Result<Self> {
        let project_root = std::env::current_dir()?;
        let dir = project_root.join(dir);

        if dir.exists() {
            Ok(Self(dir))
        } else {
            Err(anyhow::anyhow!("{} doesn't exist", dir.display()))
        }
    }
}

impl TryFrom<&str> for SettingsDirectory {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Display for SettingsDirectory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_path().display())
    }
}

impl Deref for SettingsDirectory {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<PathBuf> for SettingsDirectory {
    fn as_ref(&self) -> &PathBuf {
        &self.0
    }
}
