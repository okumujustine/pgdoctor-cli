# pgdoctor (Rust)

A local Postgres diagnostic agent written in Rust.

## Features

- Collect database metadata and performance statistics
- Query `pg_stat_statements` for top queries by execution time
- Upload snapshots to a remote API endpoint
- Query normalization and fingerprinting

## Requirements

- Rust 1.80+ (for `LazyLock` support)
- PostgreSQL with `pg_stat_statements` extension (optional)

## Building

```bash
cargo build --release
```

The binary will be at `target/release/pgdoctor`.

## Configuration

Set environment variables or create a `.env` file:

```bash
cp .env.example .env
```

| Variable | Description | Default |
|----------|-------------|---------|
| `PGDOCTOR_DSN` | PostgreSQL connection string | (required) |
| `PGDOCTOR_LIMIT` | Max queries to collect (1-200) | 10 |
| `PGDOCTOR_ENDPOINT` | API endpoint for uploads | (required for upload) |
| `PGDOCTOR_TOKEN` | API authentication token | (required for upload) |

## Usage

### Collect and print snapshot

```bash
./pgdoctor snapshot
```

### Collect and upload snapshot

```bash
./pgdoctor upload
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

## License

MIT
# pgdoctor-cli
