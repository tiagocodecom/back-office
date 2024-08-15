pub enum Environment {
    Development,
    Testing,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Testing => "testing",
            Environment::Production => "production",
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        let environment: Environment = std::env::var("APP__ENVIRONMENT")
            .unwrap_or_else(|_| "development".into())
            .try_into()
            .expect("Failed to parse APP_ENVIRONMENT.");

        environment
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" | "development" | "testing" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            other => Err(format!("{} is not a supported environment", other)),
        }
    }
}
