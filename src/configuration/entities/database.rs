use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseConfig {
    username: String,
    password: secrecy::Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    port: u16,
    host: String,
    db_name: String,
    require_ssl: bool,
}

impl DatabaseConfig {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &secrecy::Secret<String> {
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
