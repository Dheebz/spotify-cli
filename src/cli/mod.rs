pub mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "spotify-cli")]
#[command(about = "Command line interface for Spotify")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Output JSON response (silent if not specified)
    #[arg(long, short = 'j', global = true)]
    pub json: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Authentication commands
    Auth {
        #[command(subcommand)]
        command: AuthCommand,
    },
    /// Player controls
    Player {
        #[command(subcommand)]
        command: PlayerCommand,
    },
    /// Manage pinned resources
    Pin {
        #[command(subcommand)]
        command: PinCommand,
    },
    /// Search Spotify and pinned resources
    Search {
        /// Search query (can be empty if using filters)
        #[arg(default_value = "")]
        query: String,
        /// Filter by type(s): track, artist, album, playlist, show, episode, audiobook
        /// Can specify multiple: --type track --type album
        #[arg(long = "type", short = 'T')]
        types: Vec<String>,
        /// Results per type (default 20, max 50)
        #[arg(long, short = 'l', default_value = "20")]
        limit: u8,
        /// Only search pinned resources (skip Spotify API)
        #[arg(long)]
        pins_only: bool,
        /// Only show results where name contains the query
        #[arg(long, short = 'e')]
        exact: bool,
        /// Filter by artist name
        #[arg(long, short = 'a')]
        artist: Option<String>,
        /// Filter by album name
        #[arg(long, short = 'A')]
        album: Option<String>,
        /// Filter by track name
        #[arg(long, short = 't')]
        track: Option<String>,
        /// Filter by year or range (e.g., 2020 or 1990-2000)
        #[arg(long, short = 'y')]
        year: Option<String>,
        /// Filter by genre
        #[arg(long, short = 'g')]
        genre: Option<String>,
        /// Filter by ISRC code (tracks only)
        #[arg(long)]
        isrc: Option<String>,
        /// Filter by UPC code (albums only)
        #[arg(long)]
        upc: Option<String>,
        /// Only albums released in the past two weeks
        #[arg(long)]
        new: bool,
        /// Only albums with lowest 10% popularity
        #[arg(long)]
        hipster: bool,
        /// Play the first result
        #[arg(long, short = 'p')]
        play: bool,
    },
    /// Manage playlists
    Playlist {
        #[command(subcommand)]
        command: PlaylistCommand,
    },
    /// Manage your library (liked songs)
    Library {
        #[command(subcommand)]
        command: LibraryCommand,
    },
    /// Get info about track, album, or artist (defaults to now playing)
    Info {
        #[command(subcommand)]
        command: InfoCommand,
    },
    /// User profile and stats
    User {
        #[command(subcommand)]
        command: UserCommand,
    },
    /// Manage podcasts (shows)
    Show {
        #[command(subcommand)]
        command: ShowCommand,
    },
    /// Manage podcast episodes
    Episode {
        #[command(subcommand)]
        command: EpisodeCommand,
    },
    /// Manage audiobooks
    Audiobook {
        #[command(subcommand)]
        command: AudiobookCommand,
    },
    /// Get audiobook chapter details
    Chapter {
        #[command(subcommand)]
        command: ChapterCommand,
    },
    /// Browse Spotify categories
    Category {
        #[command(subcommand)]
        command: CategoryCommand,
    },
}

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
}

#[derive(Subcommand)]
pub enum PlaylistCommand {
    /// List your playlists
    List {
        /// Number of playlists to return (default 20, max 50)
        #[arg(long, short = 'l', default_value = "20")]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value = "0")]
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
    },
    /// Remove tracks from a playlist
    Remove {
        /// Playlist ID, URL, or pin alias
        playlist: String,
        /// Track URIs to remove
        #[arg(required = true)]
        uris: Vec<String>,
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
pub enum PlayerCommand {
    /// Skip to next track
    Next,
    /// Skip to previous track
    Previous,
    /// Toggle playback (play/pause)
    Toggle,
    /// Start or resume playback
    Play {
        /// Play a specific Spotify URI (track, album, playlist, etc.)
        #[arg(long, short = 'u')]
        uri: Option<String>,
        /// Play a pinned resource by alias
        #[arg(long, short = 'p')]
        pin: Option<String>,
    },
    /// Pause playback
    Pause,
    /// Get current playback status
    Status {
        /// Output only the ID (for piping): track, album, or artist
        #[arg(long, value_parser = ["track", "album", "artist"])]
        id_only: Option<String>,
    },
    /// Manage playback devices
    Devices {
        #[command(subcommand)]
        command: DevicesCommand,
    },
    /// Manage playback queue
    Queue {
        #[command(subcommand)]
        command: QueueCommand,
    },
    /// Seek to position in current track
    Seek {
        /// Position: seconds (90), time (1:30), or explicit (90s, 5000ms)
        position: String,
    },
    /// Set repeat mode
    Repeat {
        /// Repeat mode: off, track, or context
        #[arg(value_parser = ["off", "track", "context"])]
        mode: String,
    },
    /// Set playback volume
    Volume {
        /// Volume percentage (0-100)
        #[arg(value_parser = clap::value_parser!(u8).range(0..=100))]
        percent: u8,
    },
    /// Toggle shuffle mode
    Shuffle {
        /// Shuffle state: on or off
        #[arg(value_parser = ["on", "off"])]
        state: String,
    },
    /// Get recently played tracks
    Recent,
}

#[derive(Subcommand)]
pub enum DevicesCommand {
    /// List available devices
    List,
    /// Transfer playback to a device
    Transfer {
        /// Device ID or name
        device: String,
    },
}

#[derive(Subcommand)]
pub enum QueueCommand {
    /// List current queue
    List,
    /// Add item to queue
    Add {
        /// Spotify URI (e.g., spotify:track:xxx)
        uri: Option<String>,
        /// Add the currently playing track
        #[arg(long, short = 'n')]
        now_playing: bool,
    },
}

#[derive(Subcommand)]
pub enum LibraryCommand {
    /// List saved tracks (liked songs)
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
    },
    /// Remove tracks from library (unlike songs)
    Remove {
        /// Track IDs to remove
        #[arg(required = true)]
        ids: Vec<String>,
    },
    /// Check if tracks are in library
    Check {
        /// Track IDs to check
        #[arg(required = true)]
        ids: Vec<String>,
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
        #[arg(long, short = 't')]
        top_tracks: bool,
        /// Market for top tracks (ISO 3166-1 alpha-2 country code)
        #[arg(long, short = 'm', default_value = "US")]
        market: String,
    },
}

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
        #[arg(long, short = 'l', default_value = "20")]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value = "0")]
        offset: u32,
    },
    /// List saved shows
    List {
        /// Number of shows to return (default 20, max 50)
        #[arg(long, short = 'l', default_value = "20")]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value = "0")]
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
        #[arg(long, short = 'l', default_value = "20")]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value = "0")]
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
        #[arg(long, short = 'l', default_value = "20")]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value = "0")]
        offset: u32,
    },
    /// List saved audiobooks
    List {
        /// Number of audiobooks to return (default 20, max 50)
        #[arg(long, short = 'l', default_value = "20")]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value = "0")]
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
pub enum CategoryCommand {
    /// List browse categories
    List {
        /// Number of categories to return (default 20, max 50)
        #[arg(long, short = 'l', default_value = "20")]
        limit: u8,
        /// Offset for pagination
        #[arg(long, short = 'o', default_value = "0")]
        offset: u32,
    },
    /// Get category details
    Get {
        /// Category name or ID (e.g., "pop", "rock", "focus", "gaming", "dinner")
        id: String,
    },
}
