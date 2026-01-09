//! CLI parsing and command dispatch.
use clap::{Parser, Subcommand};

use crate::AppContext;
use crate::cli::auth::{AuthCommand, handle as handle_auth};
use crate::cli::completions::{CompletionsCommand, handle as handle_completions};
use crate::cli::device::{DeviceCommand, handle as handle_device};
use crate::cli::help::{HelpCommand, handle as handle_help};
use crate::cli::info::{InfoCommand, handle as handle_info};
use crate::cli::now_playing::{NowPlayingCommand, handle as handle_now_playing};
use crate::cli::pin::{PinCommand, handle as handle_pin};
use crate::cli::play::{PlayCommand, handle as handle_play};
use crate::cli::player::{PlayerCommand, handle as handle_player};
use crate::cli::playlist::{PlaylistCommand, handle as handle_playlist};
use crate::cli::queue::{QueueCommand, handle as handle_queue};
use crate::cli::recently_played::{RecentlyPlayedCommand, handle as handle_recently_played};
use crate::cli::search::{SearchCommand, handle as handle_search};
use crate::cli::sync::{SyncCommand, handle as handle_sync};
use crate::error::Result;

pub mod auth;
pub mod completions;
pub mod device;
pub mod help;
pub mod info;
pub mod now_playing;
pub mod pin;
pub mod play;
pub mod player;
pub mod playlist;
pub mod queue;
pub mod recently_played;
pub mod search;
pub mod sync;

/// Parsed CLI configuration plus resolved command.
#[derive(Debug)]
pub struct ParsedCli {
    pub json: bool,
    pub command: Command,
}

#[derive(Parser)]
#[command(name = "spotify-cli", disable_help_subcommand = true, version)]
#[command(about = "Terminal-first Spotify control surface")]
struct Cli {
    #[arg(long, global = true, help = "Output JSON")]
    json: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(subcommand)]
    Auth(AuthCommand),
    Completions(CompletionsCommand),
    #[command(subcommand)]
    Device(DeviceCommand),
    #[command(name = "help")]
    Help(HelpCommand),
    Info(InfoCommand),
    #[command(name = "nowplaying")]
    NowPlaying(NowPlayingCommand),
    #[command(subcommand)]
    Pin(PinCommand),
    #[command(hide = true)]
    Play(PlayCommand),
    #[command(subcommand)]
    Player(PlayerCommand),
    #[command(subcommand)]
    Playlist(PlaylistCommand),
    Queue(QueueCommand),
    #[command(name = "recentlyplayed")]
    RecentlyPlayed(RecentlyPlayedCommand),
    Search(SearchCommand),
    Sync(SyncCommand),
}

pub fn parse() -> ParsedCli {
    let cli = Cli::parse();
    ParsedCli {
        json: cli.json,
        command: cli.command,
    }
}

#[cfg(test)]
pub(crate) fn parse_from<I, T>(args: I) -> ParsedCli
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let cli = Cli::parse_from(args);
    ParsedCli {
        json: cli.json,
        command: cli.command,
    }
}

pub fn execute(parsed: ParsedCli, ctx: &AppContext) -> Result<()> {
    match parsed.command {
        Command::Auth(command) => handle_auth(command, ctx),
        Command::Completions(command) => handle_completions(command),
        Command::Device(command) => handle_device(command, ctx),
        Command::Help(command) => handle_help(command, ctx),
        Command::Info(command) => handle_info(command, ctx),
        Command::NowPlaying(command) => handle_now_playing(command, ctx),
        Command::Pin(command) => handle_pin(command, ctx),
        Command::Play(command) => handle_play(command, ctx),
        Command::Player(command) => handle_player(command, ctx),
        Command::Playlist(command) => handle_playlist(command, ctx),
        Command::Queue(command) => handle_queue(command, ctx),
        Command::RecentlyPlayed(command) => handle_recently_played(command, ctx),
        Command::Search(command) => handle_search(command, ctx),
        Command::Sync(command) => handle_sync(command, ctx),
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, parse_from};
    use crate::cli::search::SearchCommand;

    #[test]
    fn parse_global_flags() {
        let parsed = parse_from(["spotify-cli", "--json", "search", "all", "boards"]);
        assert!(parsed.json);
        match parsed.command {
            Command::Search(SearchCommand { .. }) => {}
            _ => panic!("expected search command"),
        }
    }

    #[test]
    fn parse_completions_command() {
        let parsed = parse_from(["spotify-cli", "completions", "zsh"]);
        match parsed.command {
            Command::Completions(_) => {}
            _ => panic!("expected completions command"),
        }
    }
}
