use qdrant_client::qdrant::point_id::PointIdOptions;
use qdrant_client::qdrant::ScoredPoint;

use crate::application::structures::SearchResult;

impl From<ScoredPoint> for SearchResult {
    fn from(point: ScoredPoint) -> Self {
        let score = point.score;

        let value = point.get("content");
        let payload = value.as_str().cloned().unwrap_or_default();

        let value = point.get("title");
        let title = value.as_str().cloned().unwrap_or_default();

        let point_id = point.id.unwrap_or_default();
        let id = point_id.point_id_options
            .map(|it| match it {
                PointIdOptions::Num(value) => value.to_string(),
                PointIdOptions::Uuid(id) => id,
            })
            .unwrap_or_default();

        SearchResult {
            id,
            score,
            title,
            payload,
        }
    }
}
