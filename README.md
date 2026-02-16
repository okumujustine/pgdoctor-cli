# pgdoctor

A local Postgres diagnostic agent written in Rust.

## Features

- Collect database metadata and performance statistics
- Query `pg_stat_statements` for top queries by execution time
- Upload snapshots to a remote API endpoint
- Query normalization and fingerprinting

## Installation

### macOS (Apple Silicon)

```bash
# Download the latest release
curl -LO https://github.com/okumujustine/pgdoctor-cli/releases/latest/download/pgdoctor-macos-arm64.tar.gz

# Extract
tar -xzf pgdoctor-macos-arm64.tar.gz

# Move to a directory in your PATH
sudo mv pgdoctor /usr/local/bin/

# Verify installation
pgdoctor --version
```

### Build from Source

Requirements:
- Rust 1.80+ (for `LazyLock` support)

```bash
# Clone the repository
git clone https://github.com/okumujustine/pgdoctor-cli.git
cd pgdoctor-cli

# Build release binary
cargo build --release

# Install globally
cargo install --path .

# Or manually move binary
sudo mv target/release/pgdoctor /usr/local/bin/
```

## Configuration

pgdoctor requires environment variables for configuration. You can set them in two ways:

### Option 1: Shell Profile (Recommended for global CLI)

Add to your `~/.zshrc` or `~/.bashrc`:

```bash
export PGDOCTOR_DSN="postgres://username:password@localhost:5432/database_name"
export PGDOCTOR_ENDPOINT="https://your-api-endpoint.com"
export PGDOCTOR_TOKEN="your-api-token"
export PGDOCTOR_LIMIT=20
```

Then reload your shell:

```bash
source ~/.zshrc
```

### Option 2: `.env` File (Per-project)

Create a `.env` file in the directory where you run pgdoctor:

```bash
PGDOCTOR_DSN=postgres://username:password@localhost:5432/database_name
PGDOCTOR_ENDPOINT=https://your-api-endpoint.com
PGDOCTOR_TOKEN=your-api-token
PGDOCTOR_LIMIT=20
```

### Environment Variables Reference

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `PGDOCTOR_DSN` | PostgreSQL connection string | Yes | - |
| `PGDOCTOR_LIMIT` | Max queries to collect (1-200) | No | 10 |
| `PGDOCTOR_ENDPOINT` | API endpoint for uploads | For `upload` command | - |
| `PGDOCTOR_TOKEN` | API authentication token | For `upload` command | - |

### PostgreSQL Connection String Format

```
postgres://[user]:[password]@[host]:[port]/[database]
```

Examples:
```bash
# Local database
PGDOCTOR_DSN="postgres://postgres:password@localhost:5432/mydb"

# Remote database with SSL
PGDOCTOR_DSN="postgres://user:pass@db.example.com:5432/mydb?sslmode=require"
```

## Prerequisites

For full functionality, enable the `pg_stat_statements` extension in PostgreSQL:

```sql
-- Connect to your database as superuser
CREATE EXTENSION IF NOT EXISTS pg_stat_statements;
```

## Usage

### Check version

```bash
pgdoctor --version
```

### Collect and print snapshot

```bash
pgdoctor snapshot
```

### Collect and upload snapshot

```bash
pgdoctor upload
```

## Example Output

```json
{
  "meta": {
    "db_name": "mydb",
    "db_version": "PostgreSQL 16.1"
  },
  "has_pg_stat_statements": true,
  "top_queries": [
    {
      "fingerprint": "abc123...",
      "normalized_query": "SELECT * FROM users WHERE id = ?",
      "calls": 1000,
      "total_exec_time_ms": 5432.1,
      "mean_exec_time_ms": 5.43,
      "rows": 1000
    }
  ]
}
```

## Troubleshooting

### "PGDOCTOR_DSN is required"
Make sure you've set the `PGDOCTOR_DSN` environment variable either in your shell profile or in a `.env` file.

### "connection refused"
Check that PostgreSQL is running and the connection string is correct.

### "pg_stat_statements not found"
The extension is optional, but to enable it run:
```sql
CREATE EXTENSION pg_stat_statements;
```

## License

MIT
