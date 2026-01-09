use anyhow::bail;
use reqwest::blocking::Client as HttpClient;
use serde::Deserialize;
use serde_json::json;

use crate::domain::device::Device;
use crate::error::Result;
use crate::spotify::auth::AuthService;
use crate::spotify::base::api_base;
use crate::spotify::error::format_api_error;

/// Spotify devices API client.
#[derive(Debug, Clone)]
pub struct DevicesClient {
    http: HttpClient,
    auth: AuthService,
}

impl DevicesClient {
    pub fn new(http: HttpClient, auth: AuthService) -> Self {
        Self { http, auth }
    }

    pub fn list(&self) -> Result<Vec<Device>> {
        let token = self.auth.token()?;
        let url = format!("{}/me/player/devices", api_base());

        let response = self.http.get(url).bearer_auth(token.access_token).send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_else(|_| "<no body>".to_string());
            bail!(format_api_error(
                "spotify devices request failed",
                status,
                &body
            ));
        }

        let payload: DevicesResponse = response.json()?;
        Ok(payload
            .devices
            .into_iter()
            .map(|device| Device {
                id: device.id,
                name: device.name,
                volume_percent: device.volume_percent,
            })
            .collect())
    }

    pub fn set_active(&self, device_id: &str) -> Result<()> {
        let token = self.auth.token()?;
        let url = format!("{}/me/player", api_base());
        let body = json!({ "device_ids": [device_id], "play": true });

        let response = self
            .http
            .put(url)
            .bearer_auth(token.access_token)
            .json(&body)
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_else(|_| "<no body>".to_string());
            bail!(format_api_error(
                "spotify device transfer failed",
                status,
                &body
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct DevicesResponse {
    devices: Vec<SpotifyDevice>,
}

#[derive(Debug, Deserialize)]
struct SpotifyDevice {
    id: String,
    name: String,
    volume_percent: Option<u32>,
}
