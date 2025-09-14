mod config;
mod dto;

pub use config::BgeConfig;

use std::sync::Arc;

use crate::application::services::{TokenizeError, TokenizeProvider, TokenizeResult};
use crate::application::structures::{TokenizedContent, TokenizedContentBuilder, TokenizerInputForm, TokensData};
use crate::infrastructure::sentransformers::dto::RequestBuilder;
use crate::ServiceConnect;

const NATIVE_SERVICE_URL: &str = "/pipeline/feature-extraction/sentence-transformers/all-MiniLM-L6-v2";

#[derive(Clone)]
pub struct BgeClient {
    config: BgeConfig,
    client: Arc<reqwest::Client>,
}

#[async_trait::async_trait]
impl ServiceConnect for BgeClient {
    type Config = BgeConfig;
    type Error = reqwest::Error;
    type Client = Self;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error> {
        tracing::debug!(url=config.address(), "connected to embeddings service");
        Ok(BgeClient {
            config: config.clone(),
            client: Arc::new(reqwest::Client::new()),
        })
    }
}

#[async_trait::async_trait]
impl TokenizeProvider for BgeClient {
    async fn compute(&self, form: &TokenizerInputForm) -> TokenizeResult<TokenizedContent> {
        let input_form = RequestBuilder::default()
            .inputs(vec![form.input().clone()])
            .build()
            .map_err(|_err| TokenizeError::EmptyResponse)?;

        let target_url = format!("{}{}", self.config.address(), NATIVE_SERVICE_URL);
        let response = self
            .client
            .clone()
            .post(target_url)
            .json(&input_form)
            .send()
            .await?;

        if !response.status().is_success() {
            let err = response
                .error_for_status()
                .err()
                .unwrap();
            return Err(TokenizeError::ServiceError(err));
        }

        let content = response.json::<Vec<Vec<Vec<Vec<f32>>>>>().await?;
        let tokens = content
            .first()
            .ok_or(TokenizeError::EmptyResponse)?
            .first()
            .ok_or(TokenizeError::EmptyResponse)?
            .first()
            .ok_or(TokenizeError::EmptyResponse)?;

        let tokens_data = TokensData::new(tokens.to_owned());
        let content = TokenizedContentBuilder::default()
            .model(form.model().clone())
            .data(vec![tokens_data])
            .build()
            .unwrap();

        Ok(content)
    }
}
