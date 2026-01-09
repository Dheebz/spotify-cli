use anyhow::{Context, bail};
use reqwest::Method;
use reqwest::blocking::Client as HttpClient;

use crate::error::Result;
use crate::spotify::auth::AuthService;
use crate::spotify::base::api_base;
use crate::spotify::error::format_api_error;

/// Spotify saved tracks (library) API client.
#[derive(Debug, Clone)]
pub struct TrackClient {
    http: HttpClient,
    auth: AuthService,
}

impl TrackClient {
    pub fn new(http: HttpClient, auth: AuthService) -> Self {
        Self { http, auth }
    }

    pub fn like(&self, track_id: &str) -> Result<()> {
        let path = format!("/me/tracks?ids={}", track_id);
        self.send(Method::PUT, &path)
    }

    pub fn unlike(&self, track_id: &str) -> Result<()> {
        let path = format!("/me/tracks?ids={}", track_id);
        self.send(Method::DELETE, &path)
    }

    fn send(&self, method: Method, path: &str) -> Result<()> {
        let token = self.auth.token()?;
        let url = format!("{}{}", api_base(), path);

        let response = self
            .http
            .request(method, url)
            .bearer_auth(token.access_token)
            .body(Vec::new())
            .send()
            .context("spotify request failed")?;

        if response.status().is_success() {
            return Ok(());
        }

        let status = response.status();
        let body = response.text().unwrap_or_else(|_| "<no body>".to_string());
        bail!(format_api_error("spotify request failed", status, &body))
    }
}
