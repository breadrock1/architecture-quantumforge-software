use derive_builder::Builder;
use gset::Getset;
use serde::{Deserialize, Serialize};

#[derive(Builder, Serialize, Getset)]
pub struct TokenizerInputForm {
    #[getset(get, vis = "pub")]
    input: String,
    #[getset(get, vis = "pub")]
    model: String,
}

#[derive(Builder, Clone, Deserialize, Getset)]
pub struct TokenizedContent {
    #[getset(get, vis = "pub")]
    model: String,
    #[getset(get, vis = "pub")]
    data: Vec<TokensData>,
}

#[derive(Clone, Deserialize, Getset)]
pub struct TokensData {
    #[getset(get, vis = "pub")]
    embedding: Vec<f32>,
}

impl TokensData {
    pub fn new(embedding: Vec<f32>) -> Self {
        TokensData { embedding }
    }
}
