//! Data models for PGDoctor

mod query;
mod response;
mod snapshot;

pub use query::QueryStat;
pub use response::IngestResponse;
pub use snapshot::{Meta, Snapshot};
