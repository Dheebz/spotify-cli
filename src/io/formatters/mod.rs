//! Formatter modules for human-readable output

pub mod album;
pub mod artist;
pub mod audiobook;
pub mod category;
pub mod chapter;
pub mod episode;
pub mod library;
pub mod pin;
pub mod player;
pub mod playlist;
pub mod search;
pub mod show;
pub mod track;
pub mod user;

pub use album::format_album_detail;
pub use artist::{format_artist_detail, format_top_artists};
pub use audiobook::{format_audiobook_chapters, format_audiobook_detail, format_audiobooks};
pub use category::{format_categories, format_category_detail};
pub use chapter::format_chapter_detail;
pub use episode::{format_episode_detail, format_episodes};
pub use library::{format_library_check, format_saved_tracks};
pub use pin::format_pins;
pub use player::{format_devices, format_player_status, format_queue};
pub use playlist::{format_playlist_detail, format_playlists};
pub use search::{format_search_results, format_spotify_search};
pub use show::{format_show_detail, format_show_episodes, format_shows};
pub use track::{format_artist_top_tracks, format_play_history, format_top_tracks, format_track_detail};
pub use user::format_user_profile;
