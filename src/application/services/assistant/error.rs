use thiserror::Error;

pub type AssistantResult<T> = Result<T, AssistantError>;

#[derive(Debug, Error)]
pub enum AssistantError {
    #[error("authentication failed: {0}")]
    AuthenticationFailed(anyhow::Error),
    #[error("connection to llm has been lost: {0}")]
    ConnectionLost(anyhow::Error),
    #[error("returned incorrect chat message from llm: {0}")]
    IncorrectResponseType(anyhow::Error),
    #[error("returned empty response from llm: {0}")]
    EmptyResponseContent(anyhow::Error),
    #[error("too many requests to llm: {0}")]
    TooManyRequests(anyhow::Error),
    #[error("validation error: {0}")]
    ValidationError(anyhow::Error),
    #[error("streaming error: {0}")]
    StreamError(anyhow::Error),
    #[error("returned undeclared error from opensearch: {0}")]
    UndeclaredError(anyhow::Error),
}
