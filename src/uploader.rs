//! HTTP client for uploading snapshots to the PGDoctor API

use anyhow::{Context, Result};
use reqwest::header;
use std::time::Duration;

use crate::models::{IngestResponse, Snapshot};

/// Default request timeout in seconds
const DEFAULT_TIMEOUT_SECS: u64 = 15;

/// HTTP client for uploading snapshots to the remote endpoint.
pub struct Client {
    endpoint: String,
    token: String,
    http: reqwest::Client,
}

impl Client {
    /// Creates a new uploader client.
    /// 
    /// # Arguments
    /// * `endpoint` - The API endpoint URL
    /// * `token` - The authentication token
    /// 
    /// # Returns
    /// A new `Client` instance
    pub fn new(endpoint: String, token: String) -> Result<Self> {
        Self::with_timeout(endpoint, token, DEFAULT_TIMEOUT_SECS)
    }

    /// Creates a new uploader client with a custom timeout.
    /// 
    /// # Arguments
    /// * `endpoint` - The API endpoint URL
    /// * `token` - The authentication token
    /// * `timeout_secs` - Request timeout in seconds
    pub fn with_timeout(endpoint: String, token: String, timeout_secs: u64) -> Result<Self> {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .context("failed to build HTTP client")?;

        Ok(Self {
            endpoint,
            token,
            http,
        })
    }

    /// Returns the configured endpoint URL.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Uploads a snapshot to the remote endpoint.
    /// 
    /// # Arguments
    /// * `snapshot` - The snapshot to upload
    /// 
    /// # Returns
    /// The response from the ingest API
    /// 
    /// # Errors
    /// Returns an error if the request fails or the response cannot be decoded
    pub async fn upload_snapshot(&self, snapshot: &Snapshot) -> Result<IngestResponse> {
        let response = self
            .http
            .post(&self.endpoint)
            .header(header::CONTENT_TYPE, "application/json")
            .bearer_auth(&self.token)
            .json(snapshot)
            .send()
            .await
            .context("failed to post snapshot")?;

        if !response.status().is_success() {
            anyhow::bail!("ingest failed with status={}", response.status());
        }

        response
            .json::<IngestResponse>()
            .await
            .context("failed to decode response")
    }
}
