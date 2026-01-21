//! Media command definitions (shows, episodes, audiobooks, chapters, albums).

use clap::Subcommand;

use crate::constants::{DEFAULT_LIMIT, DEFAULT_OFFSET};

#[derive(Subcommand)]
pub enum ShowCommand {
    /// Get show (podcast) details
    Get {
        /// Show ID or URL
        id: String,
    },
    /// List show episodes
    Episodes {
        /// Show ID or URL
        id: String,
        /// Number of episodes to return (default 20, max 50)
        #[arg(long, short = 'l', default_value_t = DEFAULT_LIMIT)]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value_t = DEFAULT_OFFSET)]
        offset: u32,
    },
    /// List saved shows
    List {
        /// Number of shows to return (default 20, max 50)
        #[arg(long, short = 'l', default_value_t = DEFAULT_LIMIT)]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value_t = DEFAULT_OFFSET)]
        offset: u32,
    },
    /// Save shows to library
    Save {
        /// Show IDs to save
        #[arg(required = true)]
        ids: Vec<String>,
    },
    /// Remove shows from library
    Remove {
        /// Show IDs to remove
        #[arg(required = true)]
        ids: Vec<String>,
    },
    /// Check if shows are in library
    Check {
        /// Show IDs to check
        #[arg(required = true)]
        ids: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum EpisodeCommand {
    /// Get episode details
    Get {
        /// Episode ID or URL
        id: String,
    },
    /// List saved episodes
    List {
        /// Number of episodes to return (default 20, max 50)
        #[arg(long, short = 'l', default_value_t = DEFAULT_LIMIT)]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value_t = DEFAULT_OFFSET)]
        offset: u32,
    },
    /// Save episodes to library
    Save {
        /// Episode IDs to save
        #[arg(required = true)]
        ids: Vec<String>,
    },
    /// Remove episodes from library
    Remove {
        /// Episode IDs to remove
        #[arg(required = true)]
        ids: Vec<String>,
    },
    /// Check if episodes are in library
    Check {
        /// Episode IDs to check
        #[arg(required = true)]
        ids: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum AudiobookCommand {
    /// Get audiobook details
    Get {
        /// Audiobook ID or URL
        id: String,
    },
    /// List audiobook chapters
    Chapters {
        /// Audiobook ID or URL
        id: String,
        /// Number of chapters to return (default 20, max 50)
        #[arg(long, short = 'l', default_value_t = DEFAULT_LIMIT)]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value_t = DEFAULT_OFFSET)]
        offset: u32,
    },
    /// List saved audiobooks
    List {
        /// Number of audiobooks to return (default 20, max 50)
        #[arg(long, short = 'l', default_value_t = DEFAULT_LIMIT)]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value_t = DEFAULT_OFFSET)]
        offset: u32,
    },
    /// Save audiobooks to library
    Save {
        /// Audiobook IDs to save
        #[arg(required = true)]
        ids: Vec<String>,
    },
    /// Remove audiobooks from library
    Remove {
        /// Audiobook IDs to remove
        #[arg(required = true)]
        ids: Vec<String>,
    },
    /// Check if audiobooks are in library
    Check {
        /// Audiobook IDs to check
        #[arg(required = true)]
        ids: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum ChapterCommand {
    /// Get chapter details
    Get {
        /// Chapter ID or URL
        id: String,
    },
}

#[derive(Subcommand)]
pub enum AlbumCommand {
    /// List saved albums
    List {
        /// Number of albums to return (default 20, max 50)
        #[arg(long, short = 'l', default_value_t = DEFAULT_LIMIT)]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value_t = DEFAULT_OFFSET)]
        offset: u32,
    },
    /// Get album tracks
    Tracks {
        /// Album ID or URL
        id: String,
        /// Number of tracks to return (default 20, max 50)
        #[arg(long, short = 'l', default_value_t = DEFAULT_LIMIT)]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value_t = DEFAULT_OFFSET)]
        offset: u32,
    },
    /// Save albums to library
    Save {
        /// Album IDs to save
        #[arg(required = true)]
        ids: Vec<String>,
    },
    /// Remove albums from library
    Remove {
        /// Album IDs to remove
        #[arg(required = true)]
        ids: Vec<String>,
    },
    /// Check if albums are in library
    Check {
        /// Album IDs to check
        #[arg(required = true)]
        ids: Vec<String>,
    },
    /// Browse new album releases
    NewReleases {
        /// Number of albums to return (default 20, max 50)
        #[arg(long, short = 'l', default_value_t = DEFAULT_LIMIT)]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value_t = DEFAULT_OFFSET)]
        offset: u32,
    },
}
