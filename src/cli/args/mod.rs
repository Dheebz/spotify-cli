//! Command argument definitions.
//!
//! This module contains all the clap subcommand enums, organized by category.

mod auth;
mod browse;
mod library;
mod media;
mod player;
mod playlist;
mod user;

pub use auth::{AuthCommand, PinCommand};
pub use browse::{CategoryCommand, FollowCommand};
pub use library::LibraryCommand;
pub use media::{AlbumCommand, AudiobookCommand, ChapterCommand, EpisodeCommand, ShowCommand};
pub use player::{DevicesCommand, PlayerCommand, QueueCommand};
pub use playlist::PlaylistCommand;
pub use user::{InfoCommand, UserCommand};
