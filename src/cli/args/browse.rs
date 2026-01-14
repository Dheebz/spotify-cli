//! Browse and follow command definitions.

use clap::Subcommand;

use crate::constants::{DEFAULT_LIMIT, DEFAULT_OFFSET};

#[derive(Subcommand)]
pub enum FollowCommand {
    /// Follow artists
    Artist {
        /// Artist IDs to follow
        #[arg(required = true)]
        ids: Vec<String>,
        /// Show what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Follow users
    User {
        /// User IDs to follow
        #[arg(required = true)]
        ids: Vec<String>,
        /// Show what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Unfollow artists
    UnfollowArtist {
        /// Artist IDs to unfollow
        #[arg(required = true)]
        ids: Vec<String>,
        /// Show what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Unfollow users
    UnfollowUser {
        /// User IDs to unfollow
        #[arg(required = true)]
        ids: Vec<String>,
        /// Show what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// List followed artists
    List {
        /// Number of artists to return (default 20, max 50)
        #[arg(long, short = 'l', default_value_t = DEFAULT_LIMIT)]
        limit: u8,
    },
    /// Check if following artists
    CheckArtist {
        /// Artist IDs to check
        #[arg(required = true)]
        ids: Vec<String>,
    },
    /// Check if following users
    CheckUser {
        /// User IDs to check
        #[arg(required = true)]
        ids: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum CategoryCommand {
    /// List browse categories
    List {
        /// Number of categories to return (default 20, max 50)
        #[arg(long, short = 'l', default_value_t = DEFAULT_LIMIT)]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value_t = DEFAULT_OFFSET)]
        offset: u32,
    },
    /// Get category details
    Get {
        /// Category name or ID (e.g., "pop", "rock", "focus", "gaming", "dinner")
        id: String,
    },
    /// Get playlists for a category
    Playlists {
        /// Category ID
        id: String,
        /// Number of playlists to return (default 20, max 50)
        #[arg(long, short = 'l', default_value_t = DEFAULT_LIMIT)]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value_t = DEFAULT_OFFSET)]
        offset: u32,
    },
}
