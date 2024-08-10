use back_office::{Application, ConfigLoader};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let configuration = ConfigLoader::from_dir().load()?;
    let application = Application::build(configuration).await?;

    application.run_until_stopped().await?;

    Ok(())
}
