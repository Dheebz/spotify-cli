//! System-level commands (sync, cache, completions).
use clap::Subcommand;

use crate::cli::cache::{handle as handle_cache, CacheCommand};
use crate::cli::completions::{handle as handle_completions, CompletionsCommand};
use crate::cli::sync::{handle as handle_sync, SyncCommand};
use crate::error::Result;
use crate::AppContext;

#[derive(Subcommand, Debug)]
pub enum SystemCommand {
    Sync(SyncCommand),
    #[command(subcommand)]
    Cache(CacheCommand),
    Completions(CompletionsCommand),
}

pub fn handle(command: SystemCommand, ctx: &AppContext) -> Result<()> {
    match command {
        SystemCommand::Sync(command) => handle_sync(command, ctx),
        SystemCommand::Cache(command) => handle_cache(command, ctx),
        SystemCommand::Completions(command) => handle_completions(command),
    }
}
