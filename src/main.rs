use back_office::{Application, ConfigLoader};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let config = ConfigLoader::from_default_dir()?;

    Application::build(config)
        .await?
        .run_until_stopped()
        .await?;

    Ok(())
}
