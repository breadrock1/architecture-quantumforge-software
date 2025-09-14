use architecture_quantumforge_software::application::services::{ServerApp, StorageProvider, TokenizeProvider};
use architecture_quantumforge_software::application::structures::{CreateDocument, TokenizerInputFormBuilder};
use architecture_quantumforge_software::application::usecase::{ChatCompletionUseCase, CHUNK_OVERLAP, CHUNK_SIZE};
use architecture_quantumforge_software::config::ServiceConfig;
use architecture_quantumforge_software::infrastructure::{BgeClient, OpenaiClient, QdrantClient};
use architecture_quantumforge_software::ServiceConnect;
use character_text_splitter::CharacterTextSplitter;
use std::fs::DirEntry;
use std::sync::Arc;

const KNOWLEDGE_DIR_PATH: &str = "./knowledge_base/sources";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = ServiceConfig::new()?;

    let qdrant_config = config.storage().qdrant();
    let storage = Arc::new(QdrantClient::connect(qdrant_config).await?);

    let bge_config = config.tokenize().bge();
    let tokenizer = Arc::new(BgeClient::connect(bge_config).await?);

    storage.delete_index().await?;
    storage.create_index().await?;

    let model_name = config.llm().openai().model_name();

    let files_content = read_sources_dir()?;
    for (file, content) in files_content {
        let text_chunks = CharacterTextSplitter::new()
            .with_chunk_size(CHUNK_SIZE)
            .with_chunk_overlap(CHUNK_OVERLAP)
            .split_text(&content);

        let doc_id = uuid::Uuid::new_v4();
        let path = file.path().as_path().to_str().unwrap().to_string();
        for chunk in text_chunks {
            let t_form = TokenizerInputFormBuilder::default()
                .input(chunk.clone())
                .model(model_name.clone())
                .build()?;

            let tokens = tokenizer.compute(&t_form).await?;
            let tokens_data = tokens.data().first().cloned().unwrap();
            let embeddings = tokens_data.embedding().to_owned();
            let s_form = CreateDocument {
                doc_id: doc_id.to_string(),
                title: path.clone(),
                file_path: path.clone(),
                content: chunk,
                embeddings,
            };

            storage.store(s_form).await?;
        }
    }

    storage.create_snapshot(config.storage().qdrant().collection().clone()).await?;

    Ok(())
}

fn read_sources_dir() -> anyhow::Result<Vec<(DirEntry, String)>> {
    let read_dir = std::fs::read_dir(KNOWLEDGE_DIR_PATH)?;
    let files_content = read_dir
        .filter_map(Result::ok)
        .filter_map(|it| {
            let path = it.path().as_path().to_str().unwrap().to_string();
            match std::fs::read_to_string(&it.path()) {
                Ok(data) => Some((it, data)),
                Err(err) => {
                    tracing::warn!(path=path, err=?err, "founded file");
                    None
                }
            }
        })
        .collect::<Vec<(DirEntry, String)>>();

    Ok(files_content)
}
