use back_office::framework::telemetry::{get_telemetry_subscriber, init_telemetry_subscriber};
use back_office::{Application, SettingsLoader};
use std::io::stdout;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_telemetry_subscriber("back-office".into(), "info".into(), stdout);
    init_telemetry_subscriber(subscriber)?;

    let config = SettingsLoader::default().load_files().deserialize()?;
    let application = Application::from(&config).await?;

    application.run_server().await?;

    Ok(())
}
