//! API response models

use serde::{Deserialize, Serialize};

/// Response from the ingest API after uploading a snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestResponse {
    /// Unique identifier for the ingested snapshot
    pub id: i64,
    /// Project identifier
    pub project_id: i64,
    /// Source identifier
    pub source_id: String,
    /// Timestamp when the snapshot was received
    pub received_at: String,
}
