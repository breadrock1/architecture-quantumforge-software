use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

use crate::application::services::{AssistantProvider, ServerApp, ServerError, ServerResult, StorageProvider, Success, TokenizeProvider};
use crate::infrastructure::httpserver::api::v1::model::CreateDocumentForm;
use crate::infrastructure::httpserver::api::v1::swagger::SwaggerExample;

pub const STORE_DOCUMENT_URL: &str = "/api/v1/storage/document";

#[utoipa::path(
    post,
    path = STORE_DOCUMENT_URL,
    tag = "storage",
    description = "Store new document to index",
    request_body(
        content_type = "application/json",
        content = CreateDocumentForm,
    ),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Returns id of stored document",
            example = "fcdc3f4d-1d79-4a32-b241-bd6ab12b1395"
        ),
        (
            status = 400,
            content_type="application/json",
            description = "Internal service error",
            body = ServerError,
            example = json!(ServerError::example(Some("storage error"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ServerError,
            example = json!(ServerError::example(None)),
        ),
    )
)]
pub async fn store_document<A, S, T>(
    State(state): State<Arc<ServerApp<A, S, T>>>,
    Json(form): Json<CreateDocumentForm>,
) -> ServerResult<impl IntoResponse>
where
    A: AssistantProvider + Sync + Send + Clone + 'static,
    S: StorageProvider + Sync + Send + Clone + 'static,
    T: TokenizeProvider + Sync + Send + Clone + 'static,
{
    let doc_id = state.get_use_case().store_document(form.into()).await?;
    Ok(Json(Success::new(200, &doc_id)))
}
