use back_office::{Application, ConfigLoader};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let config = ConfigLoader::from_default_dir()?;
    let application = Application::from(config).await?;

    application.run_server().await?;

    Ok(())
}
