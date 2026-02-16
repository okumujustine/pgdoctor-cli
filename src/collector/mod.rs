//! Snapshot collection functionality

pub mod normalizer;
mod snapshot;

pub use normalizer::{fingerprint, normalize_query};
pub use snapshot::collect;
