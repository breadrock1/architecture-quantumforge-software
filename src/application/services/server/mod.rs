mod error;

pub use error::{ServerError, ServerResult, Success};

use std::sync::Arc;

use crate::application::services::{AssistantProvider, StorageProvider, TokenizeProvider};
use crate::application::usecase::ChatCompletionUseCase;

pub struct ServerApp<A, S, T>
where
    A: AssistantProvider + Sync + Send + Clone,
    S: StorageProvider + Sync + Send + Clone,
    T: TokenizeProvider + Sync + Send + Clone,
{
    uc: Arc<ChatCompletionUseCase<A, S, T>>,
}

impl<A, S, T> ServerApp<A, S, T>
where
    A: AssistantProvider + Sync + Send + Clone,
    S: StorageProvider + Sync + Send + Clone,
    T: TokenizeProvider + Sync + Send + Clone,
{
    pub fn new(chat_use_case: Arc<ChatCompletionUseCase<A, S, T>>) -> Self {
        ServerApp { uc: chat_use_case }
    }

    pub fn get_use_case(&self) -> Arc<ChatCompletionUseCase<A, S, T>> {
        self.uc.clone()
    }
}
