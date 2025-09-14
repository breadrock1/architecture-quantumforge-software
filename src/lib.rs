pub mod application;
pub mod config;
pub mod infrastructure;
mod tracer;

pub use tracer::{LoggerConfig, init_otlp_tracing};

pub const SERVICE_NAME: &str = "rag";

#[async_trait::async_trait]
pub trait ServiceConnect {
    type Client;
    type Config;
    type Error;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error>;
}
