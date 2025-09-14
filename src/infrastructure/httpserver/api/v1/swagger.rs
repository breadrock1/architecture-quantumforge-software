use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use crate::infrastructure::httpserver::api::v1::router::assistant::*;
use crate::infrastructure::httpserver::api::v1::router::storage::*;
use crate::application::services::{ServerError, Success};

pub fn init_swagger_layer(version: &str) -> RapiDoc {
    let url_path = format!("/api/{version}/swagger");
    let config_file_path = format!("/api-docs/openapi-{version}.json");
    let swagger_app = ApiDoc::openapi();
    RapiDoc::with_openapi(config_file_path, swagger_app).path(url_path)
}

#[derive(OpenApi)]
#[openapi(
    info(
        description = "There is simple RAG system."
    ),
    tags(
        (
            name = "assistant",
            description = "LLM assistant APIs",
        ),
        (
            name = "storage",
            description = "CRUD operation for Index management",
        ),
    ),
    paths(
        ask,
        ask_stream,
        store_document,
    ),
    components(
        schemas(
            ServerError,
            Success,
        ),
    ),
)]
struct ApiDoc;

#[allow(dead_code)]
pub trait SwaggerExample {
    type Example;

    fn example(value: Option<&str>) -> Self::Example;
}

impl SwaggerExample for Success {
    type Example = Self;

    fn example(_: Option<&str>) -> Self::Example {
        Success::default()
    }
}

impl SwaggerExample for ServerError {
    type Example = Self;

    fn example(value: Option<&str>) -> Self::Example {
        match value {
            None => ServerError::ServerUnavailable,
            Some(msg) => ServerError::InternalError(msg.to_owned()),
        }
    }
}
