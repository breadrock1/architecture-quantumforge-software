mod config;
mod model;
mod error;

pub use config::QdrantConfig;

use std::sync::Arc;
use qdrant_client::{Payload, Qdrant};
use qdrant_client::qdrant::{CreateCollection, CreateSnapshotRequest, Datatype, Distance, PointStruct, SearchPoints, UpsertPointsBuilder, Vector, VectorParams, Vectors, VectorsConfig};
use qdrant_client::qdrant::vectors::VectorsOptions;
use qdrant_client::qdrant::vectors_config::Config;

use crate::application::services::{StorageProvider, StorageResult};
use crate::application::structures::{CreateDocument, SearchParams, SearchResult};
use crate::ServiceConnect;

#[derive(Clone)]
pub struct QdrantClient {
    client: Arc<Qdrant>,
    options: Arc<QdrantConfig>,
}

#[async_trait::async_trait]
impl ServiceConnect for QdrantClient {
    type Client = Self;
    type Config = QdrantConfig;
    type Error = anyhow::Error;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error> {
        let client = Qdrant::from_url(config.address())
            .api_key(Some(config.api_key().clone()))
            .build()?;

        tracing::debug!(url=config.address(), "connected to qdrant server");

        Ok(QdrantClient {
            client: Arc::new(client),
            options: Arc::new(config.to_owned()),
        })
    }
}

#[async_trait::async_trait]
impl StorageProvider for QdrantClient{
    fn get_collection(&self) -> String {
        self.options.collection().clone()
    }

    async fn store(&self, form: CreateDocument) -> StorageResult<String> {
        let id = uuid::Uuid::new_v4().to_string();

        let mut payload = Payload::new();
        payload.insert("content", form.content);

        let vector = Vector::new(form.embeddings);
        let vector_opts = VectorsOptions::Vector(vector);
        let point_struct = PointStruct::new(
            id.clone(),
            Vectors {
                vectors_options: Some(vector_opts),
            },
            payload,
        );
        let points = vec![point_struct];

        let upsert_points = UpsertPointsBuilder::new(self.options.collection(), points).build();
        let _ = self
            .client
            .upsert_points(upsert_points)
            .await?;

        Ok(id)
    }

    async fn search(&self, form: SearchParams) -> StorageResult<Vec<SearchResult>> {
        let collection_name = self.options.collection();
        let search_points = SearchPoints {
            collection_name: collection_name.clone(),
            vector: form.vector,
            limit: form.limit,
            with_payload: Some(true.into()),
            ..Default::default()
        };


        let search_response = self
            .client
            .search_points(search_points)
            .await?;

        let search_result = search_response
            .result
            .into_iter()
            .map(SearchResult::from)
            .collect::<Vec<_>>();

        Ok(search_result)
    }
}

impl QdrantClient {
    pub async fn create_index(&self) -> anyhow::Result<()> {
        let collection_name = self.options.collection();

        let vector_params = VectorParams {
            size: self.options.dimension(),
            distance: Distance::Cosine.into(),
            on_disk: Some(true),
            datatype: Some(Datatype::Float32.into()),
            ..Default::default()
            // hnsw_config: Some(HnswConfigDiff::default()),
            // quantization_config: Some(QuantizationConfig::default()),
            // multivector_config: ::core::option::Option<MultiVectorConfig>,
        };

        let vectors_config = VectorsConfig {
            config: Some(Config::Params(vector_params)),
        };

        let create_collection_options = CreateCollection {
            collection_name: collection_name.clone(),
            vectors_config: Some(vectors_config),
            ..Default::default()
        };

        let _ = self
            .client
            .create_collection(create_collection_options)
            .await?;

        Ok(())
    }

    pub async fn delete_index(&self) -> anyhow::Result<()> {
        let _ = self.client.delete_collection(self.options.collection()).await?;
        Ok(())
    }

    pub async fn create_snapshot(&self, collection: String) -> anyhow::Result<()> {
        let request = CreateSnapshotRequest {
            collection_name: collection,
        };

        let response = self
            .client
            .create_snapshot(request)
            .await?;

        println!("info={:#?} snapshot created", response);
        tracing::info!(info=?response, "snapshot created");

        Ok(())
    }
}
