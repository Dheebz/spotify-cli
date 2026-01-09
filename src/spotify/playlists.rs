use anyhow::bail;
use reqwest::blocking::Client as HttpClient;
use serde::Deserialize;

use crate::domain::playlist::{Playlist, PlaylistDetail};
use crate::error::Result;
use crate::spotify::auth::AuthService;
use crate::spotify::base::api_base;
use crate::spotify::error::format_api_error;

/// Spotify playlists API client.
#[derive(Debug, Clone)]
pub struct PlaylistsClient {
    http: HttpClient,
    auth: AuthService,
}

impl PlaylistsClient {
    pub fn new(http: HttpClient, auth: AuthService) -> Self {
        Self { http, auth }
    }

    pub fn list_all(&self) -> Result<Vec<Playlist>> {
        let token = self.auth.token()?;
        let mut url = format!("{}/me/playlists?limit=50", api_base());
        let mut playlists = Vec::new();

        loop {
            let response = self
                .http
                .get(&url)
                .bearer_auth(token.access_token.clone())
                .send()?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().unwrap_or_else(|_| "<no body>".to_string());
                bail!(format_api_error(
                    "spotify playlists request failed",
                    status,
                    &body
                ));
            }

            let payload: PlaylistsResponse = response.json()?;
            playlists.extend(payload.items.into_iter().map(|item| Playlist {
                id: item.id,
                name: item.name,
                owner: item.owner.and_then(|owner| owner.display_name),
                collaborative: item.collaborative,
                public: item.public,
            }));

            if let Some(next) = payload.next {
                url = next;
            } else {
                break;
            }
        }

        Ok(playlists)
    }

    pub fn get(&self, playlist_id: &str) -> Result<PlaylistDetail> {
        let token = self.auth.token()?;
        let url = format!("{}/playlists/{playlist_id}", api_base());

        let response = self.http.get(url).bearer_auth(token.access_token).send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_else(|_| "<no body>".to_string());
            bail!(format_api_error(
                "spotify playlist request failed",
                status,
                &body
            ));
        }

        let payload: PlaylistDetailResponse = response.json()?;
        Ok(PlaylistDetail {
            id: payload.id,
            name: payload.name,
            uri: payload.uri,
            owner: payload.owner.and_then(|owner| owner.display_name),
            tracks_total: payload.tracks.map(|tracks| tracks.total),
            collaborative: payload.collaborative,
            public: payload.public,
        })
    }

    pub fn create(&self, name: &str, public: Option<bool>) -> Result<PlaylistDetail> {
        let token = self.auth.token()?;
        let user_id = self.current_user_id(&token.access_token)?;
        let url = format!("{}/users/{user_id}/playlists", api_base());

        let mut body = serde_json::json!({ "name": name });
        if let Some(public) = public {
            body["public"] = serde_json::json!(public);
        }

        let response = self
            .http
            .post(url)
            .bearer_auth(token.access_token)
            .json(&body)
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_else(|_| "<no body>".to_string());
            bail!(format_api_error(
                "spotify playlist create failed",
                status,
                &body
            ));
        }

        let payload: PlaylistDetailResponse = response.json()?;
        Ok(PlaylistDetail {
            id: payload.id,
            name: payload.name,
            uri: payload.uri,
            owner: payload.owner.and_then(|owner| owner.display_name),
            tracks_total: payload.tracks.map(|tracks| tracks.total),
            collaborative: payload.collaborative,
            public: payload.public,
        })
    }

    pub fn rename(&self, playlist_id: &str, name: &str) -> Result<()> {
        let token = self.auth.token()?;
        let url = format!("{}/playlists/{playlist_id}", api_base());
        let body = serde_json::json!({ "name": name });

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
                "spotify playlist rename failed",
                status,
                &body
            ));
        }
        Ok(())
    }

    pub fn delete(&self, playlist_id: &str) -> Result<()> {
        self.unfollow(playlist_id)
    }

    pub fn follow(&self, playlist_id: &str) -> Result<()> {
        let token = self.auth.token()?;
        let url = format!("{}/playlists/{playlist_id}/followers", api_base());

        let response = self
            .http
            .put(url)
            .bearer_auth(token.access_token)
            .body(Vec::new())
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_else(|_| "<no body>".to_string());
            bail!(format_api_error(
                "spotify playlist follow failed",
                status,
                &body
            ));
        }
        Ok(())
    }

    pub fn unfollow(&self, playlist_id: &str) -> Result<()> {
        let token = self.auth.token()?;
        let url = format!("{}/playlists/{playlist_id}/followers", api_base());

        let response = self
            .http
            .delete(url)
            .bearer_auth(token.access_token)
            .body(Vec::new())
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_else(|_| "<no body>".to_string());
            bail!(format_api_error(
                "spotify playlist unfollow failed",
                status,
                &body
            ));
        }
        Ok(())
    }

    pub fn add_tracks(&self, playlist_id: &str, uris: &[String]) -> Result<()> {
        let token = self.auth.token()?;
        let url = format!("{}/playlists/{playlist_id}/tracks", api_base());

        let response = self
            .http
            .post(url)
            .bearer_auth(token.access_token)
            .json(&serde_json::json!({ "uris": uris }))
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_else(|_| "<no body>".to_string());
            bail!(format_api_error(
                "spotify playlist add failed",
                status,
                &body
            ));
        }
        Ok(())
    }

    fn current_user_id(&self, access_token: &str) -> Result<String> {
        let url = format!("{}/me", api_base());
        let response = self.http.get(url).bearer_auth(access_token).send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_else(|_| "<no body>".to_string());
            bail!(format_api_error(
                "spotify profile request failed",
                status,
                &body
            ));
        }

        let payload: SpotifyUser = response.json()?;
        Ok(payload.id)
    }
}

#[derive(Debug, Deserialize)]
struct PlaylistsResponse {
    items: Vec<SpotifyPlaylist>,
    next: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SpotifyPlaylist {
    id: String,
    name: String,
    owner: Option<SpotifyOwner>,
    #[serde(default)]
    collaborative: bool,
    public: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct SpotifyOwner {
    display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SpotifyUser {
    id: String,
}

#[derive(Debug, Deserialize)]
struct PlaylistDetailResponse {
    id: String,
    name: String,
    uri: String,
    owner: Option<SpotifyOwner>,
    tracks: Option<SpotifyTracks>,
    #[serde(default)]
    collaborative: bool,
    public: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct SpotifyTracks {
    total: u32,
}
