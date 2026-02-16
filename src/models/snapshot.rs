//! Snapshot models

use serde::{Deserialize, Serialize};

use super::QueryStat;

/// Database metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    /// Name of the connected database
    pub db_name: String,
    /// PostgreSQL version string
    pub db_version: String,
}

/// A complete snapshot of database statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// Database metadata
    pub meta: Meta,
    /// Whether pg_stat_statements extension is installed
    pub has_pg_stat_statements: bool,
    /// Top queries by total execution time
    pub top_queries: Vec<QueryStat>,
}
