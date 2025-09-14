mod model;
mod router;
mod swagger;

use axum::routing::post;
use axum::Router;
use std::sync::Arc;

use crate::application::services::{AssistantProvider, ServerApp, StorageProvider, TokenizeProvider};

const API_VERSION: &str = "v1";

pub fn init_v1_routers<A, S, T>() -> Router<Arc<ServerApp<A, S, T>>>
where
    A: AssistantProvider + Sync + Send + Clone + 'static,
    S: StorageProvider + Sync + Send + Clone + 'static,
    T: TokenizeProvider + Sync + Send + Clone + 'static,
{
    let swagger_layer = swagger::init_swagger_layer(API_VERSION);
    let router: Router<Arc<ServerApp<A, S, T>>> = Router::new()
        .merge(swagger_layer)
        .merge(init_assistant_layer());

    router
}

fn init_assistant_layer<A, S, T>() -> Router<Arc<ServerApp<A, S, T>>>
where
    A: AssistantProvider + Sync + Send + Clone + 'static,
    S: StorageProvider + Sync + Send + Clone + 'static,
    T: TokenizeProvider + Sync + Send + Clone + 'static,
{
    Router::new()
        .route(
            router::assistant::ASSISTANT_ASK_URL,
            post(router::assistant::ask),
        )
        .route(
            router::assistant::ASSISTANT_ASK_STREAM_URL,
            post(router::assistant::ask_stream),
        )
        .route(
            router::storage::STORE_DOCUMENT_URL,
            post(router::storage::store_document),
        )
}
