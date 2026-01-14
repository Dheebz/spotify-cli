//! Library command definitions.

use clap::Subcommand;

#[derive(Subcommand)]
pub enum LibraryCommand {
    /// List saved tracks (liked songs) (alias: ls)
    #[command(alias = "ls")]
    List {
        /// Number of tracks to return (default 20, max 50)
        #[arg(long, short = 'l', default_value = "20")]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value = "0")]
        offset: u32,
    },
    /// Save tracks to library (like songs)
    Save {
        /// Track IDs to save
        ids: Vec<String>,
        /// Save the currently playing track
        #[arg(long, short = 'n')]
        now_playing: bool,
        /// Show what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Remove tracks from library (unlike songs)
    Remove {
        /// Track IDs to remove
        #[arg(required = true)]
        ids: Vec<String>,
        /// Show what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Check if tracks are in library
    Check {
        /// Track IDs to check
        #[arg(required = true)]
        ids: Vec<String>,
    },
}
