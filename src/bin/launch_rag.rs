use architecture_quantumforge_software::application::services::ServerApp;
use architecture_quantumforge_software::application::usecase::ChatCompletionUseCase;
use architecture_quantumforge_software::config::ServiceConfig;
use architecture_quantumforge_software::infrastructure::{QdrantClient, OpenaiClient, init_server, BgeClient};
use architecture_quantumforge_software::{init_otlp_tracing, ServiceConnect};
use std::sync::Arc;
use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
use tokio::net::TcpListener;
use tower_http::{cors, trace};

#[tokio::main(worker_threads = 4)]
async fn main() -> anyhow::Result<()> {
    let config = ServiceConfig::new()?;
    let _otlp_guard = init_otlp_tracing(&config)?;

    let llm_config = config.llm().openai();
    let assistant = Arc::new(OpenaiClient::connect(llm_config).await?);

    let qdrant_config = config.storage().qdrant();
    let storage = Arc::new(QdrantClient::connect(qdrant_config).await?);

    let bge_config = config.tokenize().bge();
    let tokenizer = Arc::new(BgeClient::connect(bge_config).await?);

    let chat_use_case = Arc::new(ChatCompletionUseCase::new(assistant, storage, tokenizer));
    let server_app = ServerApp::new(chat_use_case);

    let cors_layer = cors::CorsLayer::permissive();
    let trace_layer = trace::TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO));

    let app = init_server(server_app)
        .layer(trace_layer)
        .layer(cors_layer)
        .layer(OtelAxumLayer::default());

    let server_config = config.server();
    let listener = TcpListener::bind(server_config.http().address()).await?;
    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!(err=?err, "failed to stop http server");
    };

    Ok(())
}
