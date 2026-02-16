//! Query statistics model

use serde::{Deserialize, Serialize};

/// Statistics for a single query collected from pg_stat_statements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryStat {
    /// SHA-256 fingerprint of the normalized query
    pub fingerprint: String,
    /// Normalized query with literals replaced by placeholders
    pub normalized_query: String,
    /// Number of times the query has been executed
    pub calls: i64,
    /// Total execution time in milliseconds
    pub total_exec_time_ms: f64,
    /// Mean execution time in milliseconds
    pub mean_exec_time_ms: f64,
    /// Total number of rows returned
    pub rows: i64,
}
