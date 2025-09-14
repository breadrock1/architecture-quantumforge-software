mod error;

pub use error::{AssistantError, AssistantResult};

use futures::Stream;
use std::pin::Pin;

use crate::application::structures::{CompletionContext, CompletionResult};

pub type AssistantStream = Pin<Box<dyn Stream<Item = AssistantResult<CompletionResult>> + Send>>;

#[async_trait::async_trait]
pub trait AssistantProvider {
    fn get_model_name(&self) -> String;
    async fn ask(&self, ctx: &CompletionContext) -> AssistantResult<CompletionResult>;
    async fn ask_stream(&self, ctx: &CompletionContext) -> AssistantResult<AssistantStream>;
}

