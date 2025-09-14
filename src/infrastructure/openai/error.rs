use anyhow::anyhow;
use openai_dive::v1::error::APIError;
use openai_dive::v1::resources::chat::ChatCompletionParametersBuilderError;

use crate::application::services::AssistantError;

impl From<APIError> for AssistantError {
    fn from(err: APIError) -> Self {
        match err {
            APIError::AuthenticationError(msg) => {
                let err = anyhow!("auth error: {msg}");
                AssistantError::AuthenticationFailed(err)
            }
            APIError::PermissionError(msg) => {
                let err = anyhow!("permissions error: {msg}");
                AssistantError::AuthenticationFailed(err)
            }

            APIError::InvalidRequestError(msg) => {
                let err = anyhow!("validation error: {msg}");
                AssistantError::ValidationError(err)
            }
            APIError::RateLimitError(msg) => {
                let err = anyhow!("rate limiter warning: {msg}");
                AssistantError::TooManyRequests(err)
            }
            APIError::ParseError(msg) => {
                let err = anyhow!("validation error: {msg}");
                AssistantError::ValidationError(err)
            }
            APIError::FileError(msg) => {
                let err = anyhow!("validation error: {msg}");
                AssistantError::ValidationError(err)
            }
            APIError::BadRequestError(msg) => {
                let err = anyhow!("stream error: {msg}");
                AssistantError::StreamError(err)
            }
            APIError::ServerError(msg) => {
                let err = anyhow!("stream error: {msg}");
                AssistantError::StreamError(err)
            }
            APIError::StreamError(msg) => {
                let err = anyhow!("stream error: {msg}");
                AssistantError::StreamError(err)
            }
            APIError::UnknownError(code, msg) => {
                let err = anyhow!("unknown response {code} code error: {msg}");
                AssistantError::UndeclaredError(err)
            }
            _ => {
                let err = anyhow!("stream error: {err}");
                AssistantError::StreamError(err)
            }
        }
    }
}

impl From<ChatCompletionParametersBuilderError> for AssistantError {
    fn from(err: ChatCompletionParametersBuilderError) -> Self {
        AssistantError::IncorrectResponseType(anyhow!(err))
    }
}
