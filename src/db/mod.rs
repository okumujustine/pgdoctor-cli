//! Database connection and query utilities

mod connection;
mod queries;

pub use connection::connect;
pub use queries::{collect_meta, collect_top_queries, has_pg_stat_statements};
