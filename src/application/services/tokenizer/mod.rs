mod errors;

pub use errors::{TokenizeError, TokenizeResult};

use crate::application::structures::{TokenizedContent, TokenizerInputForm};

#[async_trait::async_trait]
pub trait TokenizeProvider {
    async fn compute(&self, form: &TokenizerInputForm) -> TokenizeResult<TokenizedContent>;
}
