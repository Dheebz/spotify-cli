//! Playlist command definitions.

use clap::Subcommand;

use crate::constants::{DEFAULT_LIMIT, DEFAULT_OFFSET};

#[derive(Subcommand)]
pub enum PlaylistCommand {
    /// List your playlists (alias: ls)
    #[command(alias = "ls")]
    List {
        /// Number of playlists to return (default 20, max 50)
        #[arg(long, short = 'l', default_value_t = DEFAULT_LIMIT)]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value_t = DEFAULT_OFFSET)]
        offset: u32,
    },
    /// Get playlist details
    Get {
        /// Playlist ID or URL
        playlist: String,
    },
    /// Create a new playlist
    Create {
        /// Playlist name
        name: String,
        /// Playlist description
        #[arg(long, short = 'd')]
        description: Option<String>,
        /// Make playlist public
        #[arg(long)]
        public: bool,
    },
    /// Add tracks to a playlist
    Add {
        /// Playlist ID, URL, or pin alias
        playlist: String,
        /// Track URIs to add (e.g., spotify:track:xxx)
        uris: Vec<String>,
        /// Add the currently playing track
        #[arg(long, short = 'n')]
        now_playing: bool,
        /// Position to insert tracks (default: end)
        #[arg(long, short = 'p')]
        position: Option<u32>,
        /// Show what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Remove tracks from a playlist
    Remove {
        /// Playlist ID, URL, or pin alias
        playlist: String,
        /// Track URIs to remove
        #[arg(required = true)]
        uris: Vec<String>,
        /// Show what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Edit playlist details
    Edit {
        /// Playlist ID, URL, or pin alias
        playlist: String,
        /// New playlist name
        #[arg(long, short = 'n')]
        name: Option<String>,
        /// New playlist description
        #[arg(long, short = 'd')]
        description: Option<String>,
        /// Make playlist public
        #[arg(long, conflicts_with = "private")]
        public: bool,
        /// Make playlist private
        #[arg(long, conflicts_with = "public")]
        private: bool,
    },
    /// Reorder tracks in a playlist
    Reorder {
        /// Playlist ID, URL, or pin alias
        playlist: String,
        /// Position of first track to move (0-indexed)
        #[arg(long, short = 'f')]
        from: u32,
        /// Position to move tracks to (0-indexed)
        #[arg(long, short = 't')]
        to: u32,
        /// Number of tracks to move (default: 1)
        #[arg(long, short = 'c', default_value = "1")]
        count: u32,
    },
    /// Follow a playlist
    Follow {
        /// Playlist ID or URL
        playlist: String,
        /// Add to profile publicly
        #[arg(long)]
        public: bool,
    },
    /// Unfollow a playlist
    Unfollow {
        /// Playlist ID or URL
        playlist: String,
    },
    /// Duplicate a playlist
    Duplicate {
        /// Source playlist ID, URL, or pin alias
        playlist: String,
        /// Name for the new playlist
        #[arg(long, short = 'n')]
        name: Option<String>,
    },
    /// Get playlist cover image URL
    Cover {
        /// Playlist ID, URL, or pin alias
        playlist: String,
    },
    /// Get another user's playlists
    User {
        /// Spotify username
        user_id: String,
    },
    /// Remove duplicate tracks from a playlist (alias: dedup)
    #[command(alias = "dedup")]
    Deduplicate {
        /// Playlist ID, URL, or pin alias
        playlist: String,
        /// Show what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },
}
