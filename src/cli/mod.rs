//! CLI argument definitions and parsing.
//!
//! This module defines the command-line interface using clap.

pub mod args;
pub mod commands;

use std::io;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};

// Re-export all command types for convenience
pub use args::*;
pub use clap_complete::Shell as CompletionShell;

/// Generate shell completion script to stdout
pub fn print_completions(shell: Shell) {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "spotify-cli", &mut io::stdout());
}

#[derive(Parser)]
#[command(name = "spotify-cli")]
#[command(about = "Command line interface for Spotify")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Output JSON response (silent if not specified)
    #[arg(long, short = 'j', global = true)]
    pub json: bool,

    /// Enable verbose logging (use -vv for debug, -vvv for trace)
    #[arg(long, short = 'v', global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Log output format (pretty or json)
    #[arg(long, global = true, default_value = "pretty")]
    pub log_format: String,
}

#[derive(Subcommand)]
pub enum Command {
    /// Authentication commands
    Auth {
        #[command(subcommand)]
        command: AuthCommand,
    },
    /// Player controls (alias: p)
    #[command(alias = "p")]
    Player {
        #[command(subcommand)]
        command: PlayerCommand,
    },
    /// Manage pinned resources
    Pin {
        #[command(subcommand)]
        command: PinCommand,
    },
    /// Search Spotify and pinned resources (alias: s)
    #[command(alias = "s")]
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
    /// Manage playlists (alias: pl)
    #[command(alias = "pl")]
    Playlist {
        #[command(subcommand)]
        command: PlaylistCommand,
    },
    /// Manage your library (liked songs) (alias: lib)
    #[command(alias = "lib")]
    Library {
        #[command(subcommand)]
        command: LibraryCommand,
    },
    /// Get info about track, album, or artist (defaults to now playing) (alias: i)
    #[command(alias = "i")]
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
    /// Manage saved albums
    Album {
        #[command(subcommand)]
        command: AlbumCommand,
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
    /// Follow/unfollow artists and users
    Follow {
        #[command(subcommand)]
        command: FollowCommand,
    },
    /// List available Spotify markets (countries)
    Markets,
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}
