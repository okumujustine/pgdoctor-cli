//! CLI command handlers

use anyhow::{Context, Result};

use pgdoctor::{collect, Config, UploaderClient};

/// Package version from Cargo.toml
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Prints version information.
pub fn print_version() {
    println!("pgdoctor {}", VERSION);
}

/// Prints usage information.
pub fn print_usage() {
    eprintln!("usage: pgdoctor <command>");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  snapshot   Collect and print database statistics as JSON");
    eprintln!("  upload     Collect and upload database statistics to remote endpoint");
    eprintln!("  --version  Print version information");
    eprintln!();
    eprintln!("Environment Variables:");
    eprintln!("  PGDOCTOR_DSN       PostgreSQL connection string (required)");
    eprintln!("  PGDOCTOR_LIMIT     Maximum queries to collect (default: 10, max: 200)");
    eprintln!("  PGDOCTOR_ENDPOINT  API endpoint URL (required for upload)");
    eprintln!("  PGDOCTOR_TOKEN     API authentication token (required for upload)");
}

/// Runs the snapshot command.
pub async fn run_snapshot(cfg: &Config) -> Result<()> {
    let snap = collect(cfg).await?;
    let out = serde_json::to_string_pretty(&snap).context("failed to serialize snapshot")?;
    println!("{}", out);
    Ok(())
}

/// Runs the upload command.
pub async fn run_upload(cfg: &Config) -> Result<()> {
    if cfg.endpoint.is_empty() {
        anyhow::bail!(
            "PGDOCTOR_ENDPOINT environment variable is required for upload.\n\n\
            Set it in your shell profile (~/.zshrc):\n  \
            export PGDOCTOR_ENDPOINT=\"https://your-api-endpoint.com\"\n\n\
            Or create a .env file in your current directory:\n  \
            PGDOCTOR_ENDPOINT=https://your-api-endpoint.com"
        );
    }
    if cfg.token.is_empty() {
        anyhow::bail!(
            "PGDOCTOR_TOKEN environment variable is required for upload.\n\n\
            Set it in your shell profile (~/.zshrc):\n  \
            export PGDOCTOR_TOKEN=\"your-api-token\"\n\n\
            Or create a .env file in your current directory:\n  \
            PGDOCTOR_TOKEN=your-api-token"
        );
    }

    let snap = collect(cfg).await?;

    let client = UploaderClient::new(cfg.endpoint.clone(), cfg.token.clone())
        .context("failed to create upload client")?;

    let resp = client.upload_snapshot(&snap).await?;

    println!(
        "uploaded snapshot: id={} source_id={} received_at={}",
        resp.id, resp.source_id, resp.received_at
    );

    Ok(())
}
