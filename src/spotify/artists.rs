use anyhow::bail;
use reqwest::blocking::Client as HttpClient;
use serde::Deserialize;

use crate::domain::artist::Artist;
use crate::error::Result;
use crate::spotify::auth::AuthService;
use crate::spotify::base::api_base;
use crate::spotify::error::format_api_error;

/// Spotify artist API client.
#[derive(Debug, Clone)]
pub struct ArtistsClient {
    http: HttpClient,
    auth: AuthService,
}

impl ArtistsClient {
    pub fn new(http: HttpClient, auth: AuthService) -> Self {
        Self { http, auth }
    }

    pub fn get(&self, artist_id: &str) -> Result<Artist> {
        let token = self.auth.token()?;
        let url = format!("{}/artists/{artist_id}", api_base());

        let response = self.http.get(url).bearer_auth(token.access_token).send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_else(|_| "<no body>".to_string());
            bail!(format_api_error(
                "spotify artist request failed",
                status,
                &body
            ));
        }

        let payload: SpotifyArtist = response.json()?;
        Ok(Artist {
            id: payload.id,
            name: payload.name,
            uri: payload.uri,
            genres: payload.genres,
            followers: payload.followers.map(|followers| followers.total),
        })
    }
}

#[derive(Debug, Deserialize)]
struct SpotifyArtist {
    id: String,
    name: String,
    uri: String,
    #[serde(default)]
    genres: Vec<String>,
    followers: Option<SpotifyFollowers>,
}

#[derive(Debug, Deserialize)]
struct SpotifyFollowers {
    total: u64,
}
