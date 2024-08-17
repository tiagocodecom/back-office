use handlebars::{DirectorySourceOptions, Handlebars};

/// Returns a `Handlebars` instance with the default template directory (relative from
/// the project root).
pub fn init_handlebars_engine() -> anyhow::Result<Handlebars<'static>> {
    let mut handlebars = Handlebars::new();

    handlebars.set_dev_mode(true);
    handlebars.register_templates_directory("./views", DirectorySourceOptions::default())?;

    Ok(handlebars)
}
