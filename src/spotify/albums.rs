use anyhow::bail;
use reqwest::blocking::Client as HttpClient;
use serde::Deserialize;

use crate::domain::album::{Album, AlbumTrack};
use crate::error::Result;
use crate::spotify::auth::AuthService;
use crate::spotify::base::api_base;
use crate::spotify::error::format_api_error;

/// Spotify album API client.
#[derive(Debug, Clone)]
pub struct AlbumsClient {
    http: HttpClient,
    auth: AuthService,
}

impl AlbumsClient {
    pub fn new(http: HttpClient, auth: AuthService) -> Self {
        Self { http, auth }
    }

    pub fn get(&self, album_id: &str) -> Result<Album> {
        let token = self.auth.token()?;
        let url = format!("{}/albums/{album_id}", api_base());

        let access_token = token.access_token.clone();
        let response = self
            .http
            .get(url)
            .bearer_auth(access_token.clone())
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_else(|_| "<no body>".to_string());
            bail!(format_api_error(
                "spotify album request failed",
                status,
                &body
            ));
        }

        let payload: SpotifyAlbum = response.json()?;
        let tracks = self.fetch_tracks(album_id, &access_token)?;
        let duration_ms = tracks
            .iter()
            .map(|track| track.duration_ms as u64)
            .sum::<u64>();
        Ok(Album {
            id: payload.id,
            name: payload.name,
            uri: payload.uri,
            artists: payload
                .artists
                .into_iter()
                .map(|artist| artist.name)
                .collect(),
            release_date: payload.release_date,
            total_tracks: payload.total_tracks,
            tracks,
            duration_ms: Some(duration_ms),
        })
    }

    fn fetch_tracks(&self, album_id: &str, access_token: &str) -> Result<Vec<AlbumTrack>> {
        let mut tracks = Vec::new();
        let mut url = format!("{}/albums/{album_id}/tracks?limit=50", api_base());

        loop {
            let response = self.http.get(&url).bearer_auth(access_token).send()?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().unwrap_or_else(|_| "<no body>".to_string());
                bail!(format_api_error(
                    "spotify album tracks failed",
                    status,
                    &body
                ));
            }

            let payload: AlbumTracksResponse = response.json()?;
            tracks.extend(payload.items.into_iter().map(|item| AlbumTrack {
                name: item.name,
                duration_ms: item.duration_ms,
                track_number: item.track_number,
            }));

            if let Some(next) = payload.next {
                url = next;
            } else {
                break;
            }
        }

        Ok(tracks)
    }
}

#[derive(Debug, Deserialize)]
struct SpotifyAlbum {
    id: String,
    name: String,
    uri: String,
    release_date: Option<String>,
    total_tracks: Option<u32>,
    artists: Vec<SpotifyArtistRef>,
}

#[derive(Debug, Deserialize)]
struct SpotifyArtistRef {
    name: String,
}

#[derive(Debug, Deserialize)]
struct AlbumTracksResponse {
    items: Vec<SpotifyAlbumTrack>,
    next: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SpotifyAlbumTrack {
    name: String,
    duration_ms: u32,
    track_number: u32,
}
