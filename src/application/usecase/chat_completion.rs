use anyhow::anyhow;
use character_text_splitter::CharacterTextSplitter;
use std::sync::Arc;

use crate::application::services::{AssistantError, AssistantProvider, AssistantResult, AssistantStream};
use crate::application::services::{StorageError, StorageProvider, StorageResult};
use crate::application::services::TokenizeProvider;
use crate::application::structures::{CompletionContext, CompletionContextBuilder, CompletionResult};
use crate::application::structures::{CreateDocument, SearchParams, TokenizerInputFormBuilder};

pub const CHUNK_OVERLAP: usize = 60;
pub const CHUNK_SIZE: usize = 600;
const SEARCH_LIMIT: u64 = 2;

#[derive(Clone)]
pub struct ChatCompletionUseCase<A, S, T>
where
    A: AssistantProvider + Sync + Send,
    S: StorageProvider + Sync + Send,
    T: TokenizeProvider + Sync + Send,
{
    assistant: Arc<A>,
    storage: Arc<S>,
    tokenizer: Arc<T>,
}

impl<A, S, T> ChatCompletionUseCase<A, S, T>
where
    A: AssistantProvider + Sync + Send,
    S: StorageProvider + Sync + Send,
    T: TokenizeProvider + Sync + Send,
{
    pub fn new(assistant: Arc<A>, storage: Arc<S>, tokenizer: Arc<T>) -> Self {
        Self { assistant, storage, tokenizer }
    }

    #[tracing::instrument(skip(self))]
    pub async fn ask(&self, ctx: &CompletionContext) -> AssistantResult<CompletionResult> {
        self.assistant.ask(ctx).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn ask_stream(
        &self,
        ctx: &CompletionContext,
    ) -> AssistantResult<AssistantStream> {
        let form = TokenizerInputFormBuilder::default()
            .model(self.assistant.get_model_name())
            .input(ctx.query().clone())
            .build()
            .map_err(|err| {
                let err = anyhow!("failed to build tokens form: {err}");
                AssistantError::ValidationError(err)
            })?;

        let tokens = self
            .tokenizer
            .compute(&form)
            .await
            .map_err(|err| AssistantError::StreamError(anyhow!(err)))?;

        let embeddings = tokens
            .data()
            .first()
            .map(|it| it.embedding().clone())
            .unwrap_or_default();

        let params = SearchParams {
            collection: self.storage.get_collection(),
            vector: embeddings,
            limit: SEARCH_LIMIT,
        };

        let founded_result = self
            .storage
            .search(params)
            .await
            .map_err(|err| {
                let err = anyhow!("failed to search test tokens: {err}");
                AssistantError::StreamError(err)
            })?;

        let founded_contexts = founded_result
            .into_iter()
            .enumerate()
            .map(|(index, it)| {
                format!("[{index}] {} - {} ", it.title, it.payload.trim())
            })
            .collect::<Vec<_>>();

        let query = ctx.query();
        let common_context = founded_contexts.join("\n");
        let common_query = format!("### Documents: {common_context}\n\nQuestion: {query}");
        let ctx = CompletionContextBuilder::default()
            .query(common_query)
            .user(None)
            .attached_ctx(None)
            .prompt_kind(ctx.prompt_kind().to_owned())
            .build()
            .unwrap();

        self.assistant.ask_stream(&ctx).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn store_document(&self, doc: CreateDocument) -> StorageResult<String> {
        let text_chunks = CharacterTextSplitter::new()
            .with_chunk_size(CHUNK_SIZE)
            .with_chunk_overlap(CHUNK_OVERLAP)
            .split_text(&doc.content);

        let model_name = self.assistant.get_model_name();
        for chunk in text_chunks {
            let t_form = TokenizerInputFormBuilder::default()
                .input(chunk.clone())
                .model(model_name.clone())
                .build()
                .map_err(|err| StorageError::InternalError(anyhow!(err)))?;

            let tokens = self
                .tokenizer
                .compute(&t_form)
                .await
                .map_err(|err| StorageError::InternalError(anyhow!(err)))?;

            let tokens_data = tokens.data().first().cloned().unwrap();
            let embeddings = tokens_data.embedding();
            let s_form = CreateDocument {
                doc_id: doc.doc_id.clone(),
                title: doc.file_path.clone(),
                file_path: doc.file_path.clone(),
                content: chunk,
                embeddings: embeddings.to_owned(),
            };

            self.storage.store(s_form).await?;
        }

        Ok(doc.doc_id)
    }
}
