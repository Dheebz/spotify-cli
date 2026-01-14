//! User and info command definitions.

use clap::Subcommand;

#[derive(Subcommand)]
pub enum UserCommand {
    /// Get your profile information
    Profile,
    /// Get your top tracks or artists
    Top {
        /// What to get: tracks or artists
        #[arg(value_parser = ["tracks", "artists"])]
        item_type: String,
        /// Time range: short (4 weeks), medium (6 months), long (years)
        #[arg(long, short = 'r', default_value = "medium", value_parser = ["short", "medium", "long"])]
        range: String,
        /// Number of results (default 20, max 50)
        #[arg(long, short = 'l', default_value = "20")]
        limit: u8,
    },
    /// Get another user's profile
    Get {
        /// Spotify username
        user_id: String,
    },
}

#[derive(Subcommand)]
pub enum InfoCommand {
    /// Get track details (defaults to now playing)
    Track {
        /// Track ID or URL (optional - defaults to now playing)
        id: Option<String>,
        /// Output only the ID (for piping)
        #[arg(long)]
        id_only: bool,
    },
    /// Get album details (defaults to now playing album)
    Album {
        /// Album ID or URL (optional - defaults to now playing album)
        id: Option<String>,
        /// Output only the ID (for piping)
        #[arg(long)]
        id_only: bool,
    },
    /// Get artist details (defaults to now playing artist)
    Artist {
        /// Artist ID or URL (optional - defaults to now playing artist)
        id: Option<String>,
        /// Output only the ID (for piping)
        #[arg(long)]
        id_only: bool,
        /// Get artist's top tracks instead of details
        #[arg(long, short = 't', conflicts_with_all = ["albums", "related"])]
        top_tracks: bool,
        /// Get artist's albums instead of details
        #[arg(long, short = 'a', conflicts_with_all = ["top_tracks", "related"])]
        albums: bool,
        /// Get related artists instead of details
        #[arg(long, short = 'r', conflicts_with_all = ["top_tracks", "albums"])]
        related: bool,
        /// Market for top tracks (ISO 3166-1 alpha-2 country code)
        #[arg(long, short = 'm', default_value = "US")]
        market: String,
        /// Number of albums to return (default 20, max 50)
        #[arg(long, short = 'l', default_value = "20")]
        limit: u8,
        /// Offset for album pagination
        #[arg(long, short = 'o', default_value = "0")]
        offset: u32,
    },
}
