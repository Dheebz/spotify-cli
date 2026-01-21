//! HTTP client layer for Spotify API communication.
//!
//! ## Architecture
//!
//! - [`client`] - Low-level HTTP client wrapper using reqwest
//! - [`api`] - `SpotifyApi` for authenticated requests to api.spotify.com/v1
//! - [`auth`] - `SpotifyAuth` for token operations via accounts.spotify.com
//! - [`endpoints`] - Type-safe URL builders for all API endpoints

pub mod api;
pub mod auth;
pub mod client;
pub mod endpoints;
