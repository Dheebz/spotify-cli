#[macro_use]
mod macros;

mod albums;
mod audiobooks;
mod auth;
mod categories;
mod chapters;
mod episodes;
mod follow;
mod library;
mod markets;
pub(crate) mod now_playing;
mod pin;
mod player;
mod playlist;
mod resource;
mod search;
mod shows;
mod user;

pub use albums::*;
pub use audiobooks::*;
pub use auth::*;
pub use categories::*;
pub use chapters::*;
pub use episodes::*;
pub use follow::*;
pub use library::*;
pub use markets::*;
pub use pin::*;
pub use player::*;
pub use playlist::*;
pub use resource::*;
pub use search::*;
pub use shows::*;
pub use user::*;

/// Search filters for Spotify API field queries
#[derive(Default)]
pub struct SearchFilters {
    pub artist: Option<String>,
    pub album: Option<String>,
    pub track: Option<String>,
    pub year: Option<String>,
    pub genre: Option<String>,
    pub isrc: Option<String>,
    pub upc: Option<String>,
    pub new: bool,
    pub hipster: bool,
}

impl SearchFilters {
    /// Build the full query string with filters appended
    pub fn build_query(&self, base_query: &str) -> String {
        let mut parts: Vec<String> = Vec::new();

        if !base_query.is_empty() {
            parts.push(base_query.to_string());
        }

        if let Some(ref artist) = self.artist {
            parts.push(format!("artist:{}", artist));
        }
        if let Some(ref album) = self.album {
            parts.push(format!("album:{}", album));
        }
        if let Some(ref track) = self.track {
            parts.push(format!("track:{}", track));
        }
        if let Some(ref year) = self.year {
            parts.push(format!("year:{}", year));
        }
        if let Some(ref genre) = self.genre {
            parts.push(format!("genre:{}", genre));
        }
        if let Some(ref isrc) = self.isrc {
            parts.push(format!("isrc:{}", isrc));
        }
        if let Some(ref upc) = self.upc {
            parts.push(format!("upc:{}", upc));
        }
        if self.new {
            parts.push("tag:new".to_string());
        }
        if self.hipster {
            parts.push("tag:hipster".to_string());
        }

        parts.join(" ")
    }

    /// Check if any filters are set
    pub fn has_filters(&self) -> bool {
        self.artist.is_some()
            || self.album.is_some()
            || self.track.is_some()
            || self.year.is_some()
            || self.genre.is_some()
            || self.isrc.is_some()
            || self.upc.is_some()
            || self.new
            || self.hipster
    }
}

use std::future::Future;

use crate::http::api::SpotifyApi;
use crate::io::output::{ErrorKind, Response};
use crate::storage::config::Config;
use crate::storage::pins::{Pin, PinStore};
use crate::storage::token_store::TokenStore;

/// Initialize a TokenStore with standardized error handling
pub(crate) fn init_token_store() -> Result<TokenStore, Response> {
    TokenStore::new().map_err(|e| {
        Response::err_with_details(
            500,
            "Failed to initialize token store",
            ErrorKind::Storage,
            e.to_string(),
        )
    })
}

/// Initialize a PinStore with standardized error handling
pub(crate) fn init_pin_store() -> Result<PinStore, Response> {
    PinStore::new().map_err(|e| {
        Response::err_with_details(
            500,
            "Failed to load pin store",
            ErrorKind::Storage,
            e.to_string(),
        )
    })
}

/// Load Config with standardized error handling
pub(crate) fn load_config() -> Result<Config, Response> {
    Config::load().map_err(|e| {
        Response::err_with_details(500, "Failed to load config", ErrorKind::Config, e.to_string())
    })
}

/// Get an authenticated Spotify API client
pub(crate) async fn get_authenticated_client() -> Result<SpotifyApi, Response> {
    let token_store = init_token_store()?;

    let token = token_store.load().map_err(|_| {
        Response::err(401, "Not logged in. Run: spotify-cli auth login", ErrorKind::Auth)
    })?;

    if token.is_expired() {
        return Err(Response::err(
            401,
            "Token expired. Run: spotify-cli auth refresh",
            ErrorKind::Auth,
        ));
    }

    Ok(SpotifyApi::new(token.access_token))
}

/// Extract Spotify ID from URL or pass through if already an ID
pub(crate) fn extract_id(input: &str) -> String {
    Pin::extract_id(input)
}

/// Execute a command with an authenticated Spotify client
pub(crate) async fn with_client<F, Fut>(f: F) -> Response
where
    F: FnOnce(SpotifyApi) -> Fut,
    Fut: Future<Output = Response>,
{
    match get_authenticated_client().await {
        Ok(client) => f(client).await,
        Err(e) => e,
    }
}
