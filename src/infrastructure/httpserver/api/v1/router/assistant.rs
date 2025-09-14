use axum::extract::{Query, State};
use axum::http::Response;
use axum::response::IntoResponse;
use axum::Json;
use futures::StreamExt;
use std::sync::Arc;

use crate::application::services::{AssistantProvider, ServerApp, StorageProvider, TokenizeProvider};
use crate::application::services::{ServerError, ServerResult};
use crate::application::structures::CompletionContextBuilder;
use crate::infrastructure::httpserver::api::v1::model::{AskSimpleForm, AskSimpleQuery};
use crate::infrastructure::httpserver::api::v1::swagger::SwaggerExample;

pub const ASSISTANT_ASK_URL: &str = "/api/v1/assistant/ask";
pub const ASSISTANT_ASK_STREAM_URL: &str = "/api/v1/assistant/ask_stream";

#[utoipa::path(
    post,
    path = ASSISTANT_ASK_URL,
    tag = "assistant",
    description = "Returns llm answer from user query",
    request_body(
        content_type = "application/json",
        content = AskSimpleForm,
    ),
    responses(
        (
            status = 200,
            content_type="text/plain",
            description = "Sova AI assistant response",
            example = "Hello, I'm Sova assistant. How can i help you?"
        ),
        (
            status = 400,
            content_type="application/json",
            description = "Internal service error",
            body = ServerError,
            example = json!(ServerError::example(Some("assistant error"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ServerError,
            example = json!(ServerError::example(None)),
        ),
    )
)]
pub async fn ask<A, S, T>(
    State(state): State<Arc<ServerApp<A, S, T>>>,
    Query(query): Query<AskSimpleQuery>,
    Json(form): Json<AskSimpleForm>,
) -> ServerResult<impl IntoResponse>
where
    A: AssistantProvider + Sync + Send + Clone + 'static,
    S: StorageProvider + Sync + Send + Clone + 'static,
    T: TokenizeProvider + Sync + Send + Clone + 'static,
{
    let prompt_kind = query.kind.unwrap_or_default();
    let assistant = state.get_use_case();
    let ctx = CompletionContextBuilder::default()
        .query(form.query)
        .user(None)
        .attached_ctx(None)
        .prompt_kind(prompt_kind.into())
        .build()
        .unwrap();

    let answer = assistant.ask(&ctx).await?;
    let response = Response::builder()
        .header("Content-Type", "text/plain")
        .body(axum::body::Body::from(answer.content().clone()))
        .unwrap();

    Ok(response)
}

#[utoipa::path(
    post,
    path = ASSISTANT_ASK_STREAM_URL,
    tag = "assistant",
    description = "Returns llm chunking stream from user query",
    request_body(
        content_type = "application/json",
        content = AskSimpleForm,
    ),
    responses(
        (
            status = 200,
            content_type="text/plain",
            description = "Sova AI assistant response",
            example = "Hello, I'm Sova assistant. How can i help you?"
        ),
        (
            status = 400,
            content_type="application/json",
            description = "Internal service error",
            body = ServerError,
            example = json!(ServerError::example(Some("assistant error"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ServerError,
            example = json!(ServerError::example(None)),
        ),
    )
)]
pub async fn ask_stream<A, S, T>(
    State(state): State<Arc<ServerApp<A, S, T>>>,
    Query(query): Query<AskSimpleQuery>,
    Json(form): Json<AskSimpleForm>,
) -> ServerResult<impl IntoResponse>
where
    A: AssistantProvider + Sync + Send + Clone + 'static,
    S: StorageProvider + Sync + Send + Clone + 'static,
    T: TokenizeProvider + Sync + Send + Clone + 'static,
{
    let prompt_kind = query.kind.unwrap_or_default();
    let assistant = state.get_use_case();
    let ctx = CompletionContextBuilder::default()
        .query(form.query)
        .user(None)
        .attached_ctx(None)
        .prompt_kind(prompt_kind.into())
        .build()
        .unwrap();

    let stream = assistant.ask_stream(&ctx).await?
        .map(|it| {
            it.map(|i| i.content().clone())
        })
        .boxed();

    let response = Response::builder()
        .header("Content-Type", "text/plain")
        .header("Transfer-Encoding", "chunked")
        .body(axum::body::Body::from_stream(stream))
        .unwrap();

    Ok(response)
}
