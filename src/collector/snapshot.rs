//! Snapshot collection from PostgreSQL database

use anyhow::Result;

use crate::config::Config;
use crate::db;
use crate::models::{Meta, Snapshot};

/// Collects a snapshot of database statistics.
///
/// This function connects to the database, collects metadata,
/// and optionally collects query statistics from pg_stat_statements.
///
/// # Arguments
/// * `cfg` - The configuration containing database connection details
///
/// # Returns
/// A `Snapshot` containing the collected data
///
/// # Errors
/// Returns an error if:
/// - The DSN is not provided
/// - The database connection fails
/// - Metadata collection fails
pub async fn collect(cfg: &Config) -> Result<Snapshot> {
    if cfg.dsn.is_empty() {
        anyhow::bail!(
            "PGDOCTOR_DSN environment variable is required.\n\n\
            Set it in your shell profile (~/.zshrc):\n  \
            export PGDOCTOR_DSN=\"postgres://user:password@localhost:5432/dbname\"\n\n\
            Or create a .env file in your current directory:\n  \
            PGDOCTOR_DSN=postgres://user:password@localhost:5432/dbname\n\n\
            Run 'pgdoctor --help' for more information."
        );
    }

    let client = db::connect(&cfg.dsn).await?;

    let (db_name, db_version) = db::collect_meta(&client).await?;
    let has_pss = db::has_pg_stat_statements(&client).await?;

    let mut snap = Snapshot {
        meta: Meta {
            db_name,
            db_version,
        },
        has_pg_stat_statements: has_pss,
        top_queries: vec![],
    };

    if has_pss {
        match db::collect_top_queries(&client, cfg.limit).await {
            Ok(queries) => snap.top_queries = queries,
            Err(e) => {
                // If this fails (permissions, view missing), keep snapshot usable.
                eprintln!("warning: failed to collect top queries: {}", e);
            }
        }
    }

    Ok(snap)
}
