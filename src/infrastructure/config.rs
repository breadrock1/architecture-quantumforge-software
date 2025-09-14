use gset::Getset;
use serde_derive::Deserialize;

use crate::infrastructure::httpserver::HttpServerConfig;
use crate::infrastructure::openai::OpenaiConfig;
use crate::infrastructure::sentransformers::BgeConfig;
use crate::infrastructure::qdrant::QdrantConfig;

#[derive(Clone, Deserialize, Getset)]
pub struct TokenizerConfig {
    #[getset(get, vis = "pub")]
    bge: BgeConfig,
}

#[derive(Clone, Deserialize, Getset)]
pub struct LlmConfig {
    #[getset(get, vis = "pub")]
    openai: OpenaiConfig,
}

#[derive(Clone, Deserialize, Getset)]
pub struct StorageConfig {
    #[getset(get, vis = "pub")]
    qdrant: QdrantConfig,
}

#[derive(Clone, Deserialize, Getset)]
pub struct ServerConfig {
    #[getset(get, vis = "pub")]
    http: HttpServerConfig,
}
