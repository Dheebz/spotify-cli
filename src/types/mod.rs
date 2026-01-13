//! Typed response structures for Spotify API.
//!
//! These types mirror the Spotify Web API response formats and provide
//! type-safe access to response data instead of raw JSON.

mod album;
mod artist;
mod common;
mod playback;
mod playlist;
mod track;
mod user;

pub use album::*;
pub use artist::*;
pub use common::*;
pub use playback::*;
pub use playlist::*;
pub use track::*;
pub use user::*;
