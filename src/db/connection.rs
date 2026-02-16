//! Database connection handling

use anyhow::{Context, Result};
use tokio_postgres::{Client, NoTls};

/// Establishes a connection to the PostgreSQL database.
///
/// # Arguments
/// * `dsn` - The database connection string (DSN)
///
/// # Returns
/// A connected `Client` instance
pub async fn connect(dsn: &str) -> Result<Client> {
    let (client, connection) = tokio_postgres::connect(dsn, NoTls)
        .await
        .context("failed to connect to database")?;

    // Spawn connection handler in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}
