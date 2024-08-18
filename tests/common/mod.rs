use back_office::framework::database::Database;
use back_office::framework::settings::entities::config::Config;
use back_office::framework::settings::entities::database::DatabaseConfig;
use back_office::framework::telemetry::{get_telemetry_subscriber, init_telemetry_subscriber};
use back_office::{Application, SettingsLoader};
use once_cell::sync::Lazy;
use reqwest::redirect::Policy;
use reqwest::{Client, Method, Response};
use serde_json::Value;
use sqlx::{migrate, Connection, Executor, PgConnection, PgPool};
use std::env::var;
use std::io::{sink, stdout};
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    if var("TEST_LOG").is_ok() {
        let subscriber = get_telemetry_subscriber("test".into(), "info".into(), stdout);
        init_telemetry_subscriber(subscriber).unwrap();
    } else {
        let subscriber = get_telemetry_subscriber("test".into(), "info".into(), sink);
        init_telemetry_subscriber(subscriber).unwrap();
    };
});

/// A struct representing the test application.
pub struct TestApplication {
    pub address: String,
    pub http_client: Client,
    pub connection_pool: PgPool,
}

impl TestApplication {
    pub async fn request_json(&self, method: Method, path: &str, body: Value) -> Response {
        self.http_client
            .request(method, &format!("{}{}", self.address, path))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_json(&self, path: &str, body: Value) -> Response {
        self.request_json(Method::POST, path, body).await
    }

    pub async fn get_json(&self, path: &str, body: Value) -> Response {
        self.request_json(Method::GET, path, body).await
    }

    pub async fn get(&self, path: &str) -> Response {
        self.http_client
            .get(&format!("{}{}", self.address, path))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_test_app() -> TestApplication {
    Lazy::force(&TRACING);

    let config = spawn_test_app_config();
    let http_client = spawn_test_http_client();
    let connection_pool = spawn_test_database_pool(&config.database).await;

    let application = Application::from(&config).await.unwrap();
    let port = application.port();
    let _ = tokio::spawn(application.run_server());

    TestApplication {
        http_client,
        connection_pool,
        address: format!("http://localhost:{}", port),
    }
}

/// Spawns a test application configuration.
/// This configuration assigns a random database name and a random port,
/// so that the tests do not interfere with each other.
fn spawn_test_app_config() -> Config {
    let config = {
        let mut config = SettingsLoader::default()
            .load_files()
            .deserialize()
            .unwrap();

        config.database.db_name = Uuid::new_v4().to_string();
        config.application.port = 0;
        config
    };

    config
}

/// Spawns a test database pool.
/// This pool creates a new database with a random name and runs the migrations.
/// The database is dropped when the pool is dropped.
async fn spawn_test_database_pool(config: &DatabaseConfig) -> PgPool {
    let database = Database::from(config);

    let mut connection =
        PgConnection::connect_with(&database.options_without_db_name().database("postgres"))
            .await
            .expect("Failed to connect to the default database");

    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.db_name))
        .await
        .expect("Failed to create the test database");

    let connection_pool = PgPool::connect_with(database.options_with_db_name())
        .await
        .expect("Failed to connect to Postgres");

    migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to run the database migrations");

    connection_pool
}

/// Spawns a test HTTP client.
/// This client is used to make HTTP requests to during the tests.
fn spawn_test_http_client() -> Client {
    Client::builder()
        .redirect(Policy::none())
        .cookie_store(true)
        .build()
        .unwrap()
}
