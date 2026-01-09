//! Output formatting for human and JSON modes.
use crate::domain::album::Album;
use crate::domain::artist::Artist;
use crate::domain::auth::{AuthScopes, AuthStatus};
use crate::domain::cache::CacheStatus;
use crate::domain::device::Device;
use crate::domain::pin::PinnedPlaylist;
use crate::domain::player::PlayerStatus;
use crate::domain::playlist::{Playlist, PlaylistDetail};
use crate::domain::search::{SearchItem, SearchResults};
use crate::domain::settings::Settings;
use crate::domain::track::Track;
use crate::error::Result;

pub mod cache;
pub mod human;
pub mod json;
pub mod pin;
pub mod settings;

/// Output mode for CLI responses.
#[derive(Debug, Clone, Copy)]
pub enum OutputMode {
    Human,
    Json,
}

pub const DEFAULT_MAX_WIDTH: usize = 48;

/// Table rendering configuration for human output.
#[derive(Debug, Clone, Copy)]
pub struct TableConfig {
    pub max_width: Option<usize>,
    pub truncate: bool,
}

/// Unified output facade for CLI commands.
#[derive(Debug, Clone)]
pub struct Output {
    mode: OutputMode,
    user_name: Option<String>,
    table: TableConfig,
}

impl Output {
    pub fn new(
        json: bool,
        user_name: Option<String>,
        max_width: Option<usize>,
        no_trunc: bool,
    ) -> Self {
        let mode = if json {
            OutputMode::Json
        } else {
            OutputMode::Human
        };
        let table = TableConfig {
            max_width,
            truncate: !no_trunc,
        };
        Self {
            mode,
            user_name,
            table,
        }
    }

    pub fn auth_status(&self, status: AuthStatus) -> Result<()> {
        match self.mode {
            OutputMode::Human => human::auth_status(status),
            OutputMode::Json => json::auth_status(status),
        }
    }

    pub fn auth_scopes(&self, scopes: AuthScopes) -> Result<()> {
        match self.mode {
            OutputMode::Human => human::auth_scopes(scopes),
            OutputMode::Json => json::auth_scopes(scopes),
        }
    }

    pub fn player_status(&self, status: PlayerStatus) -> Result<()> {
        match self.mode {
            OutputMode::Human => human::player_status(status),
            OutputMode::Json => json::player_status(status),
        }
    }

    pub fn now_playing(&self, status: PlayerStatus) -> Result<()> {
        match self.mode {
            OutputMode::Human => human::now_playing(status),
            OutputMode::Json => json::now_playing(status),
        }
    }

    pub fn search_results(&self, results: SearchResults) -> Result<()> {
        match self.mode {
            OutputMode::Human => human::search_results(results, self.table),
            OutputMode::Json => json::search_results(results),
        }
    }

    pub fn queue(&self, now_playing_id: Option<&str>, items: Vec<Track>) -> Result<()> {
        match self.mode {
            OutputMode::Human => human::queue(items, now_playing_id, self.table),
            OutputMode::Json => {
                let items = items
                    .into_iter()
                    .map(|track| {
                        let id = track.id;
                        crate::domain::search::SearchItem {
                            id: id.clone(),
                            name: track.name,
                            uri: format!("spotify:track:{}", id),
                            kind: crate::domain::search::SearchType::Track,
                            artists: track.artists,
                            album: track.album,
                            duration_ms: track.duration_ms,
                            owner: None,
                            score: None,
                        }
                    })
                    .collect();
                json::queue(now_playing_id, items)
            }
        }
    }

    pub fn recently_played(
        &self,
        now_playing_id: Option<&str>,
        items: Vec<SearchItem>,
    ) -> Result<()> {
        match self.mode {
            OutputMode::Human => human::recently_played(items, now_playing_id, self.table),
            OutputMode::Json => json::recently_played(now_playing_id, items),
        }
    }

    pub fn cache_status(&self, status: CacheStatus) -> Result<()> {
        match self.mode {
            OutputMode::Human => cache::status_human(status),
            OutputMode::Json => cache::status_json(status),
        }
    }

    pub fn action(&self, event: &str, message: &str) -> Result<()> {
        match self.mode {
            OutputMode::Human => human::action(message),
            OutputMode::Json => json::action(event, message),
        }
    }

    pub fn album_info(&self, album: Album) -> Result<()> {
        match self.mode {
            OutputMode::Human => human::album_info(album, self.table),
            OutputMode::Json => json::album_info(album),
        }
    }

    pub fn artist_info(&self, artist: Artist) -> Result<()> {
        match self.mode {
            OutputMode::Human => human::artist_info(artist),
            OutputMode::Json => json::artist_info(artist),
        }
    }

    pub fn playlist_list(&self, playlists: Vec<Playlist>) -> Result<()> {
        match self.mode {
            OutputMode::Human => {
                human::playlist_list(playlists, self.user_name.as_deref(), self.table)
            }
            OutputMode::Json => json::playlist_list(playlists),
        }
    }

    pub fn playlist_list_with_pins(
        &self,
        playlists: Vec<Playlist>,
        pins: Vec<PinnedPlaylist>,
    ) -> Result<()> {
        match self.mode {
            OutputMode::Human => human::playlist_list_with_pins(
                playlists,
                pins,
                self.user_name.as_deref(),
                self.table,
            ),
            OutputMode::Json => json::playlist_list_with_pins(playlists, pins),
        }
    }

    pub fn playlist_info(&self, playlist: PlaylistDetail) -> Result<()> {
        match self.mode {
            OutputMode::Human => human::playlist_info(playlist, self.user_name.as_deref()),
            OutputMode::Json => json::playlist_info(playlist),
        }
    }

    pub fn device_list(&self, devices: Vec<Device>) -> Result<()> {
        match self.mode {
            OutputMode::Human => human::device_list(devices, self.table),
            OutputMode::Json => json::device_list(devices),
        }
    }

    pub fn settings(&self, settings: Settings) -> Result<()> {
        match self.mode {
            OutputMode::Human => settings::settings_human(settings),
            OutputMode::Json => settings::settings_json(settings),
        }
    }

    pub fn pin_list(&self, pins: Vec<PinnedPlaylist>) -> Result<()> {
        match self.mode {
            OutputMode::Human => pin::pin_list_human(pins, self.table),
            OutputMode::Json => pin::pin_list_json(pins),
        }
    }

    pub fn help(&self) -> Result<()> {
        match self.mode {
            OutputMode::Human => human::help(),
            OutputMode::Json => json::help(),
        }
    }
}
