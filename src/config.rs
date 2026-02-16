//! Configuration management for PGDoctor

use std::env;

/// Default query limit
const DEFAULT_LIMIT: i32 = 10;
/// Minimum allowed query limit
const MIN_LIMIT: i32 = 1;
/// Maximum allowed query limit
const MAX_LIMIT: i32 = 200;

/// Environment variable names
mod env_vars {
    pub const DSN: &str = "PGDOCTOR_DSN";
    pub const LIMIT: &str = "PGDOCTOR_LIMIT";
    pub const ENDPOINT: &str = "PGDOCTOR_ENDPOINT";
    pub const TOKEN: &str = "PGDOCTOR_TOKEN";
}

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    /// PostgreSQL connection string (DSN)
    pub dsn: String,
    /// Maximum number of queries to collect (1-200)
    pub limit: i32,
    /// Remote API endpoint URL for uploading snapshots
    pub endpoint: String,
    /// Authentication token for the API
    pub token: String,
}

impl Config {
    /// Loads configuration from environment variables.
    /// 
    /// # Environment Variables
    /// - `PGDOCTOR_DSN` - PostgreSQL connection string (required for snapshot/upload)
    /// - `PGDOCTOR_LIMIT` - Maximum queries to collect (default: 10, range: 1-200)
    /// - `PGDOCTOR_ENDPOINT` - API endpoint URL (required for upload)
    /// - `PGDOCTOR_TOKEN` - API authentication token (required for upload)
    pub fn load() -> Self {
        let limit = Self::parse_limit();

        Self {
            dsn: env::var(env_vars::DSN).unwrap_or_default(),
            limit,
            endpoint: env::var(env_vars::ENDPOINT).unwrap_or_default(),
            token: env::var(env_vars::TOKEN).unwrap_or_default(),
        }
    }

    /// Checks if the configuration has a valid DSN.
    pub fn has_dsn(&self) -> bool {
        !self.dsn.is_empty()
    }

    /// Checks if the configuration has upload credentials.
    pub fn has_upload_credentials(&self) -> bool {
        !self.endpoint.is_empty() && !self.token.is_empty()
    }

    /// Parses and validates the limit from environment variable.
    fn parse_limit() -> i32 {
        env::var(env_vars::LIMIT)
            .ok()
            .and_then(|v| v.parse::<i32>().ok())
            .filter(|&n| (MIN_LIMIT..=MAX_LIMIT).contains(&n))
            .unwrap_or(DEFAULT_LIMIT)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::load()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_limit() {
        // When PGDOCTOR_LIMIT is not set, should use default
        let config = Config {
            dsn: String::new(),
            limit: DEFAULT_LIMIT,
            endpoint: String::new(),
            token: String::new(),
        };
        assert_eq!(config.limit, 10);
    }

    #[test]
    fn test_has_dsn() {
        let mut config = Config::default();
        assert!(!config.has_dsn());
        
        config.dsn = "postgres://localhost/test".to_string();
        assert!(config.has_dsn());
    }

    #[test]
    fn test_has_upload_credentials() {
        let mut config = Config::default();
        assert!(!config.has_upload_credentials());
        
        config.endpoint = "https://api.example.com".to_string();
        assert!(!config.has_upload_credentials());
        
        config.token = "secret-token".to_string();
        assert!(config.has_upload_credentials());
    }
}
