mod api;
mod error;

mod config;

pub use config::HttpServerConfig;

use axum::Router;
use std::sync::Arc;

use crate::application::services::{AssistantProvider, StorageProvider, TokenizeProvider};
use crate::application::services::ServerApp;
use crate::infrastructure::httpserver::api::init_v1_routers;

const SWAGGER_CONFIG_FILE: &str = "/api-docs/openapi.json";

pub fn init_server<A, S, T>(app: ServerApp<A, S, T>) -> Router
where
    A: AssistantProvider + Sync + Send + Clone + 'static,
    S: StorageProvider + Sync + Send + Clone + 'static,
    T: TokenizeProvider + Sync + Send + Clone + 'static,
{
    let app_arc = Arc::new(app);
    Router::new()
        .merge(init_v1_routers())
        .with_state(app_arc)
}


