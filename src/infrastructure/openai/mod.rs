mod config;
mod error;

pub use config::OpenaiConfig;

use anyhow::anyhow;
use futures::StreamExt;
use openai_dive::v1::api::Client;
use openai_dive::v1::error::APIError;
use openai_dive::v1::resources::chat::{
    ChatCompletionChunkResponse, ChatCompletionParametersBuilder, ChatCompletionResponseFormat,
    ChatMessage, ChatMessageContent, DeltaChatMessage,
};
use std::sync::Arc;

use crate::application::services::{AssistantError, AssistantProvider, AssistantResult, AssistantStream};
use crate::application::structures::{CompletionContext, CompletionResult, CompletionResultBuilder};
use crate::ServiceConnect;

const OPENAI_PROJECT_NAME: &str = "rag-assistant";
const OPENAI_ORGANIZATION_NAME: &str = "rag";
const ASSISTANT_SYSTEM_PROMPT: &str = "system";
const ASSISTANT_USER_PROMPT: &str = "user";

type LlmResponse = Result<ChatCompletionChunkResponse, APIError>;

#[derive(Clone)]
pub struct OpenaiClient {
    client: Arc<Client>,
    options: Arc<OpenaiConfig>,
}

#[async_trait::async_trait]
impl ServiceConnect for OpenaiClient {
    type Config = OpenaiConfig;
    type Client = Self;
    type Error = anyhow::Error;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error> {
        let mut client = Client::new(config.api_key().to_owned());
        client
            .set_base_url(config.base_url())
            .set_project(OPENAI_PROJECT_NAME)
            .set_organization(OPENAI_ORGANIZATION_NAME);

        tracing::debug!(url=config.base_url(), "connected to openai llm");

        Ok(OpenaiClient {
            client: Arc::new(client),
            options: Arc::new(config.to_owned()),
        })
    }
}

#[async_trait::async_trait]
impl AssistantProvider for OpenaiClient {
    fn get_model_name(&self) -> String {
        self.options.model_name().clone()
    }

    #[tracing::instrument(skip(self))]
    async fn ask(&self, ctx: &CompletionContext) -> AssistantResult<CompletionResult> {
        let system_prompt = ctx.prompt_kind().get_prompt();
        let messages = vec![
            ChatMessage::System {
                name: Some(ASSISTANT_SYSTEM_PROMPT.to_owned()),
                content: ChatMessageContent::Text(system_prompt.to_owned()),
            },
            ChatMessage::User {
                name: Some(ASSISTANT_USER_PROMPT.to_owned()),
                content: ChatMessageContent::Text(ctx.query().to_owned()),
            }
        ];

        tracing::debug!(prompt=system_prompt, query=ctx.query(), "sending request to llm");

        let default_user = String::from("guest");
        let completion_params = ChatCompletionParametersBuilder::default()
            .user(ctx.user().as_ref().unwrap_or(&default_user))
            .model(self.options.model_name())
            .messages(messages)
            .response_format(ChatCompletionResponseFormat::Text)
            .build()?;

        let response = self
            .client
            .chat()
            .create(completion_params)
            .await?;

        let choice = response
            .choices
            .first()
            .ok_or(AssistantError::StreamError(anyhow!("empty response")))?;

        let ChatMessage::Assistant {
            content,
            name,
            ..
        } = &choice.message else {
            return Err(AssistantError::StreamError(anyhow!("empty response")));
        };

        let answer = content.clone().unwrap().to_string();
        let completion = CompletionResultBuilder::default()
            .tool_calls(None)
            .object(response.object)
            .model(response.model)
            .created(response.created)
            .name(name.clone())
            .content(answer)
            .tool_call_id(None)
            .refusal(None)
            .reasoning_content(None)
            .build()
            .unwrap();

        Ok(completion)
    }

    #[tracing::instrument(skip(self))]
    async fn ask_stream(&self, ctx: &CompletionContext) -> AssistantResult<AssistantStream> {
        let system_prompt = ctx.prompt_kind().get_prompt();
        let messages = vec![
            ChatMessage::System {
                name: Some(ASSISTANT_SYSTEM_PROMPT.to_owned()),
                content: ChatMessageContent::Text(system_prompt.to_owned()),
            },
            ChatMessage::User {
                name: Some(ASSISTANT_USER_PROMPT.to_owned()),
                content: ChatMessageContent::Text(ctx.query().to_owned()),
            }
        ];

        tracing::debug!(prompt=system_prompt, query=ctx.query(), "sending request to llm");

        let default_user = String::from("guest");
        let completion_params = ChatCompletionParametersBuilder::default()
            .user(ctx.user().as_ref().unwrap_or(&default_user))
            .model(self.options.model_name())
            .messages(messages)
            .response_format(ChatCompletionResponseFormat::Text)
            .stream(true)
            .build()?;

        let stream = self
            .client
            .chat()
            .create_stream(completion_params)
            .await?
            .map(Self::extract_chunk_response)
            .boxed();

        Ok(stream)
    }
}

impl OpenaiClient {
    fn extract_chunk_response(response: LlmResponse) -> AssistantResult<CompletionResult> {
        let chunk_response = response
            .map_err(|err| {
                let err = anyhow!("api error: {err}");
                AssistantError::IncorrectResponseType(err)
            })?;

        let mut binding = CompletionResultBuilder::default();
        let result_builder = binding
            .tool_calls(None)
            .object(chunk_response.object)
            .model(chunk_response.model)
            .created(chunk_response.created);

        if let Some(chunk) = chunk_response.choices.first() {
            match &chunk.delta {
                DeltaChatMessage::Developer { content, name } => {
                    result_builder
                        .name(name.to_owned())
                        .content(content.to_string());
                }
                DeltaChatMessage::System { content, name } => {
                    result_builder
                        .name(name.to_owned())
                        .content(content.to_string());
                }
                DeltaChatMessage::User { content, name } => {
                    result_builder
                        .name(name.to_owned())
                        .content(content.to_string());
                }
                DeltaChatMessage::Tool { content, tool_call_id } => {
                    result_builder
                        .name(Some(tool_call_id.to_owned()))
                        .content(content.to_string());
                }
                DeltaChatMessage::Assistant {
                    content,
                    reasoning_content,
                    refusal,
                    name,
                    ..
                } => {
                    let content_str = content.as_ref().map(|it| it.to_string()).unwrap_or_default();
                    result_builder
                        .name(name.to_owned())
                        .content(content_str)
                        .tool_call_id(None)
                        .refusal(refusal.to_owned())
                        .reasoning_content(reasoning_content.to_owned());
                }
                DeltaChatMessage::Untagged {
                    content,
                    reasoning_content,
                    refusal,
                    name,
                    tool_call_id,
                    ..
                } => {
                    let content_str = content.as_ref().map(|it| it.to_string()).unwrap_or_default();
                    result_builder
                        .name(name.to_owned())
                        .content(content_str)
                        .tool_call_id(tool_call_id.clone())
                        .refusal(refusal.to_owned())
                        .reasoning_content(reasoning_content.to_owned());
                }
            }
        }

        let result = result_builder.build().unwrap();
        Ok(result)
    }
}

