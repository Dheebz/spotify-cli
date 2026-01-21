//! Player-related command definitions.

use clap::Subcommand;

#[derive(Subcommand)]
pub enum PlayerCommand {
    /// Skip to next track (alias: n)
    #[command(alias = "n")]
    Next,
    /// Skip to previous track (alias: prev)
    #[command(alias = "prev")]
    Previous,
    /// Toggle playback (play/pause) (alias: t)
    #[command(alias = "t")]
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
    /// Get current playback status (alias: st)
    #[command(alias = "st")]
    Status {
        /// Output only the ID (for piping): track, album, or artist
        #[arg(long, value_parser = ["track", "album", "artist"])]
        id_only: Option<String>,
    },
    /// Manage playback devices (alias: dev)
    #[command(alias = "dev")]
    Devices {
        #[command(subcommand)]
        command: DevicesCommand,
    },
    /// Manage playback queue (alias: q)
    #[command(alias = "q")]
    Queue {
        #[command(subcommand)]
        command: QueueCommand,
    },
    /// Seek to position in current track
    Seek {
        /// Position: seconds (90), time (1:30), or explicit (90s, 5000ms)
        position: String,
    },
    /// Set repeat mode (alias: rep)
    #[command(alias = "rep")]
    Repeat {
        /// Repeat mode: off, track, or context
        #[arg(value_parser = ["off", "track", "context"])]
        mode: String,
    },
    /// Set playback volume (alias: vol)
    #[command(alias = "vol")]
    Volume {
        /// Volume percentage (0-100)
        #[arg(value_parser = clap::value_parser!(u8).range(0..=100))]
        percent: u8,
    },
    /// Toggle shuffle mode (alias: sh)
    #[command(alias = "sh")]
    Shuffle {
        /// Shuffle state: on or off
        #[arg(value_parser = ["on", "off"])]
        state: String,
    },
    /// Get recently played tracks (alias: rec)
    #[command(alias = "rec")]
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
    /// List current queue (alias: ls)
    #[command(alias = "ls")]
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
