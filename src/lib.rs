//! PGDoctor CLI - PostgreSQL diagnostics collector
//!
//! This library provides functionality to collect PostgreSQL database statistics
//! and upload them to a remote endpoint for analysis.

pub mod collector;
pub mod config;
pub mod db;
pub mod models;
pub mod uploader;

// Re-export commonly used types for convenience
pub use collector::collect;
pub use config::Config;
pub use models::{IngestResponse, Meta, QueryStat, Snapshot};
pub use uploader::Client as UploaderClient;
