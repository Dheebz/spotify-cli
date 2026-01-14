//! CLI argument definitions and parsing.
//!
//! This module defines the command-line interface using clap.

pub mod args;
pub mod commands;

use std::io;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};

use crate::constants::DEFAULT_LIMIT;

// Re-export all command types for convenience
pub use args::*;
pub use clap_complete::Shell as CompletionShell;

/// Generate shell completion script to stdout
pub fn print_completions(shell: Shell) {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "spotify-cli", &mut io::stdout());
}

#[derive(Parser)]
#[command(name = "spotify-cli", version)]
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
        #[arg(long, short = 'l', default_value_t = DEFAULT_LIMIT)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_auth_login() {
        let cli = Cli::try_parse_from(["spotify-cli", "auth", "login"]).unwrap();
        match cli.command {
            Command::Auth { command: AuthCommand::Login { force } } => {
                assert!(!force);
            }
            _ => panic!("Expected Auth Login command"),
        }
    }

    #[test]
    fn parse_auth_login_force() {
        let cli = Cli::try_parse_from(["spotify-cli", "auth", "login", "-f"]).unwrap();
        match cli.command {
            Command::Auth { command: AuthCommand::Login { force } } => {
                assert!(force);
            }
            _ => panic!("Expected Auth Login command"),
        }
    }

    #[test]
    fn parse_player_next() {
        let cli = Cli::try_parse_from(["spotify-cli", "player", "next"]).unwrap();
        match cli.command {
            Command::Player { command: PlayerCommand::Next } => {}
            _ => panic!("Expected Player Next command"),
        }
    }

    #[test]
    fn parse_player_alias_p() {
        let cli = Cli::try_parse_from(["spotify-cli", "p", "next"]).unwrap();
        match cli.command {
            Command::Player { command: PlayerCommand::Next } => {}
            _ => panic!("Expected Player Next command via alias"),
        }
    }

    #[test]
    fn parse_player_volume() {
        let cli = Cli::try_parse_from(["spotify-cli", "player", "volume", "50"]).unwrap();
        match cli.command {
            Command::Player { command: PlayerCommand::Volume { percent } } => {
                assert_eq!(percent, 50);
            }
            _ => panic!("Expected Player Volume command"),
        }
    }

    #[test]
    fn parse_player_volume_max() {
        let cli = Cli::try_parse_from(["spotify-cli", "player", "volume", "100"]).unwrap();
        match cli.command {
            Command::Player { command: PlayerCommand::Volume { percent } } => {
                assert_eq!(percent, 100);
            }
            _ => panic!("Expected Player Volume command"),
        }
    }

    #[test]
    fn parse_player_volume_invalid() {
        let result = Cli::try_parse_from(["spotify-cli", "player", "volume", "101"]);
        assert!(result.is_err());
    }

    #[test]
    fn parse_search_default() {
        let cli = Cli::try_parse_from(["spotify-cli", "search", "test query"]).unwrap();
        match cli.command {
            Command::Search { query, limit, pins_only, exact, .. } => {
                assert_eq!(query, "test query");
                assert_eq!(limit, 20);
                assert!(!pins_only);
                assert!(!exact);
            }
            _ => panic!("Expected Search command"),
        }
    }

    #[test]
    fn parse_search_with_options() {
        let cli = Cli::try_parse_from([
            "spotify-cli", "search", "query",
            "--type", "track",
            "--limit", "10",
            "--pins-only",
            "--exact",
        ]).unwrap();
        match cli.command {
            Command::Search { query, types, limit, pins_only, exact, .. } => {
                assert_eq!(query, "query");
                assert_eq!(types, vec!["track"]);
                assert_eq!(limit, 10);
                assert!(pins_only);
                assert!(exact);
            }
            _ => panic!("Expected Search command"),
        }
    }

    #[test]
    fn parse_search_alias_s() {
        let cli = Cli::try_parse_from(["spotify-cli", "s", "query"]).unwrap();
        match cli.command {
            Command::Search { query, .. } => {
                assert_eq!(query, "query");
            }
            _ => panic!("Expected Search command via alias"),
        }
    }

    #[test]
    fn parse_json_flag() {
        let cli = Cli::try_parse_from(["spotify-cli", "-j", "markets"]).unwrap();
        assert!(cli.json);
    }

    #[test]
    fn parse_verbose_flag() {
        let cli = Cli::try_parse_from(["spotify-cli", "-v", "markets"]).unwrap();
        assert_eq!(cli.verbose, 1);
    }

    #[test]
    fn parse_verbose_multiple() {
        let cli = Cli::try_parse_from(["spotify-cli", "-vvv", "markets"]).unwrap();
        assert_eq!(cli.verbose, 3);
    }

    #[test]
    fn parse_log_format() {
        let cli = Cli::try_parse_from(["spotify-cli", "--log-format", "json", "markets"]).unwrap();
        assert_eq!(cli.log_format, "json");
    }

    #[test]
    fn parse_pin_add() {
        let cli = Cli::try_parse_from([
            "spotify-cli", "pin", "add", "track", "spotify:track:123", "my alias"
        ]).unwrap();
        match cli.command {
            Command::Pin { command: PinCommand::Add { resource_type, url_or_id, alias, tags } } => {
                assert_eq!(resource_type, "track");
                assert_eq!(url_or_id, "spotify:track:123");
                assert_eq!(alias, "my alias");
                assert!(tags.is_none());
            }
            _ => panic!("Expected Pin Add command"),
        }
    }

    #[test]
    fn parse_pin_add_with_tags() {
        let cli = Cli::try_parse_from([
            "spotify-cli", "pin", "add", "playlist", "123", "alias", "-t", "tag1,tag2"
        ]).unwrap();
        match cli.command {
            Command::Pin { command: PinCommand::Add { tags, .. } } => {
                assert_eq!(tags, Some("tag1,tag2".to_string()));
            }
            _ => panic!("Expected Pin Add command"),
        }
    }

    #[test]
    fn parse_playlist_list() {
        let cli = Cli::try_parse_from(["spotify-cli", "playlist", "list"]).unwrap();
        match cli.command {
            Command::Playlist { command: PlaylistCommand::List { limit, offset } } => {
                assert_eq!(limit, 20);
                assert_eq!(offset, 0);
            }
            _ => panic!("Expected Playlist List command"),
        }
    }

    #[test]
    fn parse_library_alias() {
        let cli = Cli::try_parse_from(["spotify-cli", "lib", "list"]).unwrap();
        match cli.command {
            Command::Library { command: LibraryCommand::List { .. } } => {}
            _ => panic!("Expected Library List command via alias"),
        }
    }

    #[test]
    fn parse_info_alias() {
        let cli = Cli::try_parse_from(["spotify-cli", "i", "track"]).unwrap();
        match cli.command {
            Command::Info { command: InfoCommand::Track { .. } } => {}
            _ => panic!("Expected Info Track command via alias"),
        }
    }

    #[test]
    fn parse_markets() {
        let cli = Cli::try_parse_from(["spotify-cli", "markets"]).unwrap();
        match cli.command {
            Command::Markets => {}
            _ => panic!("Expected Markets command"),
        }
    }

    #[test]
    fn parse_player_repeat() {
        let cli = Cli::try_parse_from(["spotify-cli", "player", "repeat", "track"]).unwrap();
        match cli.command {
            Command::Player { command: PlayerCommand::Repeat { mode } } => {
                assert_eq!(mode, "track");
            }
            _ => panic!("Expected Player Repeat command"),
        }
    }

    #[test]
    fn parse_player_shuffle() {
        let cli = Cli::try_parse_from(["spotify-cli", "player", "shuffle", "on"]).unwrap();
        match cli.command {
            Command::Player { command: PlayerCommand::Shuffle { state } } => {
                assert_eq!(state, "on");
            }
            _ => panic!("Expected Player Shuffle command"),
        }
    }

    #[test]
    fn parse_player_seek() {
        let cli = Cli::try_parse_from(["spotify-cli", "player", "seek", "1:30"]).unwrap();
        match cli.command {
            Command::Player { command: PlayerCommand::Seek { position } } => {
                assert_eq!(position, "1:30");
            }
            _ => panic!("Expected Player Seek command"),
        }
    }

    #[test]
    fn parse_user_top() {
        let cli = Cli::try_parse_from(["spotify-cli", "user", "top", "tracks", "-r", "short"]).unwrap();
        match cli.command {
            Command::User { command: UserCommand::Top { item_type, range, limit } } => {
                assert_eq!(item_type, "tracks");
                assert_eq!(range, "short");
                assert_eq!(limit, 20);
            }
            _ => panic!("Expected User Top command"),
        }
    }

    #[test]
    fn parse_user_top_default_range() {
        let cli = Cli::try_parse_from(["spotify-cli", "user", "top", "artists"]).unwrap();
        match cli.command {
            Command::User { command: UserCommand::Top { item_type, range, limit } } => {
                assert_eq!(item_type, "artists");
                assert_eq!(range, "medium");
                assert_eq!(limit, 20);
            }
            _ => panic!("Expected User Top command"),
        }
    }

    #[test]
    fn parse_follow_artist() {
        let cli = Cli::try_parse_from(["spotify-cli", "follow", "artist", "123"]).unwrap();
        match cli.command {
            Command::Follow { command: FollowCommand::Artist { ids, dry_run } } => {
                assert_eq!(ids, vec!["123"]);
                assert!(!dry_run);
            }
            _ => panic!("Expected Follow Artist command"),
        }
    }
}
