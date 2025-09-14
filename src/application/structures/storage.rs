#[derive(Debug)]
pub struct Document {
    pub id: String,
    pub chunk_id: String,
    pub title: String,
    pub file_path: String,
    pub content: String,
    pub embeddings: Vec<f32>,
}

#[derive(Debug)]
pub struct CreateDocument {
    pub doc_id: String,
    pub title: String,
    pub file_path: String,
    pub content: String,
    pub embeddings: Vec<f32>,
}

#[derive(Debug)]
pub struct SearchParams {
    pub collection: String,
    pub vector: Vec<f32>,
    pub limit: u64,
}

#[derive(Debug)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub title: String,
    pub payload: String,
}
