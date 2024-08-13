use anyhow::Context;
use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

/// Initializes the global telemetry subscriber.
///
/// Sets up `LogTracer` to route logs from `log` to `tracing` and sets the provided `Subscriber` as the global default.
///
/// # Arguments
///
/// * `subscriber` - Subscriber to set as the global default.
///
/// # Returns
///
/// `Ok(())` if successful, otherwise an error.
///
/// # Example
///
/// ```rust
/// use std::io::stdout;
/// use back_office::framework::telemetry::{get_telemetry_subscriber, init_telemetry_subscriber};
///
/// let subscriber = get_telemetry_subscriber("my_app".to_string(), "info".to_string(), stdout);
/// init_telemetry_subscriber(subscriber).expect("Failed to initialize telemetry");
/// ```
///
/// # Errors
///
/// Returns an error if `LogTracer` or setting the global default fails.
pub fn init_telemetry_subscriber(subscriber: impl Subscriber + Send + Sync) -> anyhow::Result<()> {
    LogTracer::init().context("Failed to set the log tracer")?;
    set_global_default(subscriber).context("Failed to set the subscriber")?;
    Ok(())
}

/// Creates a telemetry subscriber with structured logging.
///
/// This subscriber includes:
/// - An environment filter controlled by `RUST_LOG` or the provided `env`.
/// - JSON storage for structured logs.
/// - Bunyan formatting for output.
///
/// # Arguments
///
/// * `name` - Application or service name for Bunyan logs.
/// * `level` - Default log level (e.g., "info") if `RUST_LOG` is unset.
/// * `sink` - Output writer (e.g., `stdout`).
///
/// # Returns
///
/// A `Subscriber` with the configured layers.
///
/// # Example
///
/// ```rust
/// use std::io::stdout;
/// use back_office::framework::telemetry::get_telemetry_subscriber;
///
/// let subscriber = get_telemetry_subscriber("my_app".to_string(), "info".to_string(), stdout);
/// ```
///
/// # Panics
///
/// Panics if both `RUST_LOG` and `env` are invalid.
pub fn get_telemetry_subscriber<Sink>(
    name: String,
    level: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}
