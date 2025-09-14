mod error;

pub use error::{StorageError, StorageResult};

use crate::application::structures::{CreateDocument, SearchParams, SearchResult};

#[async_trait::async_trait]
pub trait StorageProvider {
    fn get_collection(&self) -> String;
    async fn store(&self, form: CreateDocument) -> StorageResult<String>;
    async fn search(&self, form: SearchParams) -> StorageResult<Vec<SearchResult>>;
}
