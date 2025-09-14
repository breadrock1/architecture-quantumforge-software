use gset::Getset;
use serde::Deserialize;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;

use crate::config::ServiceConfig;

#[derive(Clone, Deserialize, Getset)]
pub struct LoggerConfig {
    #[getset(get, vis = "pub")]
    level: String,
}

#[allow(unused_mut)]
pub fn init_otlp_tracing(config: &ServiceConfig) -> anyhow::Result<()> {
    init_rust_log_env(config.logger());

    let env_filter = tracing_subscriber::EnvFilter::from_default_env();
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_span_events(FmtSpan::NEW)
        .pretty();

    let subscriber = tracing_subscriber::Registry::default()
        .with(env_filter)
        .with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

fn init_rust_log_env(config: &LoggerConfig) {
    let level = config.level();
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", level);
        }
    }
}
