//! Shared Spotify API base URL configuration.

pub const DEFAULT_API_BASE: &str = "https://api.spotify.com/v1";

pub fn api_base() -> String {
    let base =
        std::env::var("SPOTIFY_CLI_API_BASE").unwrap_or_else(|_| DEFAULT_API_BASE.to_string());
    base.trim_end_matches('/').to_string()
}
