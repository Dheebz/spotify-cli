use anyhow::{Context, bail};
use reqwest::Method;
use reqwest::blocking::Client as HttpClient;
use serde::Deserialize;
use serde_json::json;

use crate::domain::device::Device;
use crate::domain::player::{PlaybackContext, PlayerStatus};
use crate::domain::track::Track;
use crate::error::Result;
use crate::spotify::auth::AuthService;
use crate::spotify::base::api_base;
use crate::spotify::error::format_api_error;

/// Spotify playback API client.
#[derive(Debug, Clone)]
pub struct PlaybackClient {
    http: HttpClient,
    auth: AuthService,
}

#[derive(Debug)]
pub struct QueueState {
    pub now_playing: Option<Track>,
    pub queue: Vec<Track>,
}

impl PlaybackClient {
    pub fn new(http: HttpClient, auth: AuthService) -> Self {
        Self { http, auth }
    }

    pub fn play(&self) -> Result<()> {
        self.send(Method::PUT, "/me/player/play", None)
    }

    pub fn pause(&self) -> Result<()> {
        self.send(Method::PUT, "/me/player/pause", None)
    }

    pub fn next(&self) -> Result<()> {
        self.send(Method::POST, "/me/player/next", None)
    }

    pub fn previous(&self) -> Result<()> {
        self.send(Method::POST, "/me/player/previous", None)
    }

    pub fn play_context(&self, uri: &str) -> Result<()> {
        let body = json!({ "context_uri": uri });
        self.send(Method::PUT, "/me/player/play", Some(body))
    }

    pub fn play_track(&self, uri: &str) -> Result<()> {
        let body = json!({ "uris": [uri] });
        self.send(Method::PUT, "/me/player/play", Some(body))
    }

    pub fn status(&self) -> Result<PlayerStatus> {
        let token = self.auth.token()?;
        let url = format!("{}/me/player", api_base());

        let response = self
            .http
            .get(url)
            .bearer_auth(token.access_token)
            .send()
            .context("spotify status request failed")?;

        if response.status() == reqwest::StatusCode::NO_CONTENT {
            return Ok(PlayerStatus {
                is_playing: false,
                track: None,
                device: None,
                context: None,
                progress_ms: None,
                repeat_state: None,
                shuffle_state: None,
            });
        }

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_else(|_| "<no body>".to_string());
            bail!(format_api_error("spotify status failed", status, &body));
        }

        let payload: SpotifyPlayerStatus = response.json()?;
        Ok(payload.into())
    }

    pub fn shuffle(&self, state: bool) -> Result<()> {
        let path = format!("/me/player/shuffle?state={}", state);
        self.send(Method::PUT, &path, None)
    }

    pub fn repeat(&self, state: &str) -> Result<()> {
        let path = format!("/me/player/repeat?state={}", state);
        self.send(Method::PUT, &path, None)
    }

    pub fn set_volume(&self, percent: u32) -> Result<()> {
        let path = format!("/me/player/volume?volume_percent={}", percent);
        self.send(Method::PUT, &path, None)
    }

    pub fn queue(&self, limit: u32) -> Result<QueueState> {
        let token = self.auth.token()?;
        let url = format!("{}/me/player/queue", api_base());

        let response = self
            .http
            .get(url)
            .bearer_auth(token.access_token)
            .send()
            .context("spotify queue request failed")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_else(|_| "<no body>".to_string());
            bail!(format_api_error("spotify queue failed", status, &body));
        }

        let payload: SpotifyQueueResponse = response.json()?;
        let now_playing = payload.currently_playing.and_then(map_track);
        let mut queue = Vec::new();
        for track in payload.queue {
            if let Some(track) = map_track(track) {
                queue.push(track);
            }
            if queue.len() >= limit as usize {
                break;
            }
        }
        Ok(QueueState { now_playing, queue })
    }

    fn send(&self, method: Method, path: &str, body: Option<serde_json::Value>) -> Result<()> {
        let token = self.auth.token()?;
        let url = format!("{}{}", api_base(), path);

        let mut request = self
            .http
            .request(method, url)
            .bearer_auth(token.access_token);
        if let Some(body) = body {
            request = request.json(&body);
        } else {
            request = request.body(Vec::new());
        }

        let response = request.send().context("spotify request failed")?;

        if response.status().is_success() {
            return Ok(());
        }

        let status = response.status();
        let body = response.text().unwrap_or_else(|_| "<no body>".to_string());
        bail!(format_api_error("spotify request failed", status, &body))
    }
}

#[derive(Debug, Deserialize)]
struct SpotifyPlayerStatus {
    #[serde(default)]
    is_playing: bool,
    progress_ms: Option<u32>,
    item: Option<SpotifyTrack>,
    device: Option<SpotifyDevice>,
    context: Option<SpotifyContext>,
    repeat_state: Option<String>,
    shuffle_state: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct SpotifyTrack {
    id: Option<String>,
    name: String,
    duration_ms: Option<u32>,
    album: Option<SpotifyAlbum>,
    artists: Vec<SpotifyArtist>,
}

#[derive(Debug, Deserialize)]
struct SpotifyArtist {
    id: Option<String>,
    name: String,
}

#[derive(Debug, Deserialize)]
struct SpotifyDevice {
    id: String,
    name: String,
    volume_percent: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct SpotifyAlbum {
    id: Option<String>,
    name: String,
}

#[derive(Debug, Deserialize)]
struct SpotifyContext {
    #[serde(rename = "type")]
    kind: Option<String>,
    uri: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SpotifyQueueResponse {
    currently_playing: Option<SpotifyTrack>,
    #[serde(default)]
    queue: Vec<SpotifyTrack>,
}

impl From<SpotifyPlayerStatus> for PlayerStatus {
    fn from(value: SpotifyPlayerStatus) -> Self {
        let track = value.item.and_then(|item| {
            item.id.map(|id| {
                let (album, album_id) = match item.album {
                    Some(album) => (Some(album.name), album.id),
                    None => (None, None),
                };

                Track {
                    id,
                    name: item.name,
                    album,
                    album_id,
                    artists: item.artists.iter().map(|a| a.name.clone()).collect(),
                    artist_ids: item.artists.into_iter().filter_map(|a| a.id).collect(),
                    duration_ms: item.duration_ms,
                }
            })
        });

        let device = value.device.map(|device| Device {
            id: device.id,
            name: device.name,
            volume_percent: device.volume_percent,
        });

        let context = value.context.and_then(|context| {
            let kind = context.kind?;
            let uri = context.uri?;
            Some(PlaybackContext { kind, uri })
        });

        PlayerStatus {
            is_playing: value.is_playing,
            track,
            device,
            context,
            progress_ms: value.progress_ms,
            repeat_state: value.repeat_state,
            shuffle_state: value.shuffle_state,
        }
    }
}

fn map_track(item: SpotifyTrack) -> Option<Track> {
    item.id.map(|id| {
        let (album, album_id) = match item.album {
            Some(album) => (Some(album.name), album.id),
            None => (None, None),
        };

        Track {
            id,
            name: item.name,
            album,
            album_id,
            artists: item.artists.iter().map(|a| a.name.clone()).collect(),
            artist_ids: item.artists.into_iter().filter_map(|a| a.id).collect(),
            duration_ms: item.duration_ms,
        }
    })
}
