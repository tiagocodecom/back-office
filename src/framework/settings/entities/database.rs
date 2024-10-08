use secrecy::Secret;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Clone, Debug)]
pub struct DatabaseConfig {
    username: String,
    password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    port: u16,
    host: String,
    pub db_name: String,
    require_ssl: bool,
}

impl DatabaseConfig {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &Secret<String> {
        &self.password
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn db_name(&self) -> &str {
        &self.db_name
    }

    pub fn require_ssl(&self) -> bool {
        self.require_ssl
    }
}
