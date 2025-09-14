mod assistant;

pub use assistant::CompletionFile;
pub use assistant::{CompletionContext, CompletionContextBuilder, CompletionContextBuilderError};
pub use assistant::{CompletionResult, CompletionResultBuilder, CompletionResultBuilderError};

mod prompt;

pub use prompt::PromptKind;
pub use prompt::{SYSTEM_SIMPLE_PROMPT, SYSTEM_PROMPT_FEW_SHOT, SYSTEM_PROMPT_CHAIN_OF_THOUGHT};

mod storage;

pub use storage::{Document, CreateDocument};
pub use storage::{SearchParams, SearchResult};

mod tokenizer;

pub use tokenizer::{TokenizerInputForm, TokenizerInputFormBuilder, TokenizerInputFormBuilderError};
pub use tokenizer::{TokenizedContent, TokenizedContentBuilder, TokenizedContentBuilderError};
pub use tokenizer::TokensData;
