use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::application::structures::{CreateDocument, PromptKind};

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct AskSimpleForm {
    pub query: String,
    pub prompt: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct AskSimpleQuery {
    pub kind: Option<PromptKindForm>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub enum PromptKindForm {
    #[default]
    Simple,
    FewShot,
    ChainOfThought,
    Custom(String),
}

impl From<PromptKindForm> for PromptKind {
    fn from(form: PromptKindForm) -> Self {
        match form {
            PromptKindForm::Simple => PromptKind::Simple,
            PromptKindForm::FewShot => PromptKind::FewShot,
            PromptKindForm::ChainOfThought => PromptKind::ChainOfThought,
            PromptKindForm::Custom(content) => PromptKind::Custom(content),
        }
    }
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct CreateDocumentForm {
    title: String,
    file_path: String,
    content: String,
}

impl From<CreateDocumentForm> for CreateDocument {
    fn from(form: CreateDocumentForm) -> Self {
        let doc_id = uuid::Uuid::new_v4();
        CreateDocument {
            doc_id: doc_id.to_string(),
            title: form.title.clone(),
            file_path: form.file_path.clone(),
            content: form.content.clone(),
            embeddings: Vec::default(),
        }
    }
}
