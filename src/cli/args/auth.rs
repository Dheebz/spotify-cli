//! Authentication command definitions.

use clap::Subcommand;

#[derive(Subcommand)]
pub enum AuthCommand {
    /// Login to Spotify (opens browser for OAuth)
    Login {
        /// Force re-authentication (new browser flow)
        #[arg(long, short = 'f')]
        force: bool,
    },
    /// Logout and clear stored tokens
    Logout,
    /// Refresh the access token
    Refresh,
    /// Check authentication status
    Status,
}

#[derive(Subcommand)]
pub enum PinCommand {
    /// Add a pinned resource
    Add {
        /// Resource type: playlist, track, album, artist, show, episode, audiobook
        #[arg(value_parser = ["playlist", "track", "album", "artist", "show", "episode", "audiobook"])]
        resource_type: String,
        /// Spotify URL or ID
        url_or_id: String,
        /// Human-friendly alias for searching
        alias: String,
        /// Optional comma-separated tags
        #[arg(long, short = 't')]
        tags: Option<String>,
    },
    /// Remove a pinned resource
    Remove {
        /// Alias or ID of the pin to remove
        alias_or_id: String,
    },
    /// List pinned resources
    List {
        /// Filter by resource type
        #[arg(long, short = 'T', value_parser = ["playlist", "track", "album", "artist", "show", "episode", "audiobook"])]
        resource_type: Option<String>,
    },
}
