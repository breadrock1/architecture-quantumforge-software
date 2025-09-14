use derive_builder::Builder;
use gset::Getset;
use serde::Serialize;

use crate::application::structures::prompt::PromptKind;

#[derive(Builder, Clone, Debug, Getset)]
pub struct CompletionContext {
    #[getset(get, vis = "pub")]
    query: String,
    #[getset(get, vis = "pub")]
    user: Option<String>,
    #[getset(get, vis = "pub")]
    attached_ctx: Option<CompletionFile>,
    #[getset(get, vis = "pub")]
    prompt_kind: PromptKind,
}

#[derive(Clone, Debug, Getset)]
pub struct CompletionFile {
    #[getset(get, vis = "pub")]
    name: String,
    #[getset(get, vis = "pub")]
    mime: String,
    #[getset(get, vis = "pub")]
    data: Vec<u8>,

}

impl CompletionFile {
    pub fn new(name: String, mime: String, data: Vec<u8>) -> Self {
        CompletionFile { name, mime, data }
    }
}

#[derive(Builder, Debug, Getset, Serialize)]
pub struct CompletionResult {
    #[getset(get, vis = "pub")]
    object: String,
    #[getset(get, vis = "pub")]
    model: String,
    #[getset(get_copy, vis = "pub")]
    created: u32,
    #[getset(get, vis = "pub")]
    content: String,
    #[getset(get, vis = "pub")]
    name: Option<String>,
    #[getset(get, vis = "pub")]
    refusal: Option<String>,
    #[getset(get, vis = "pub")]
    tool_calls: Option<String>,
    #[getset(get, vis = "pub")]
    tool_call_id: Option<String>,
    #[getset(get, vis = "pub")]
    reasoning_content: Option<String>,
}
