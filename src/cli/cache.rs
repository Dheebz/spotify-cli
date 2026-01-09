//! Cache command handlers.
use clap::Subcommand;

use crate::domain::cache::CacheStatus;
use crate::error::Result;
use crate::AppContext;

#[derive(Subcommand, Debug)]
pub enum CacheCommand {
    Status,
    Country { code: Option<String> },
    User { name: Option<String> },
}

pub fn handle(command: CacheCommand, ctx: &AppContext) -> Result<()> {
    match command {
        CacheCommand::Status => status(ctx),
        CacheCommand::Country { code } => country(ctx, code),
        CacheCommand::User { name } => user(ctx, name),
    }
}

fn status(ctx: &AppContext) -> Result<()> {
    let devices = ctx.cache.device_cache().load()?;
    let playlists = ctx.cache.playlist_cache().load()?;

    let device_count = devices.as_ref().map(|snap| snap.items.len()).unwrap_or(0);
    let playlist_count = playlists.as_ref().map(|snap| snap.items.len()).unwrap_or(0);

    let status = CacheStatus {
        root: ctx.cache.root().display().to_string(),
        device_count,
        playlist_count,
    };
    ctx.output.cache_status(status)
}

fn country(ctx: &AppContext, code: Option<String>) -> Result<()> {
    if let Some(code) = code {
        ctx.auth.set_country(Some(code))?;
    }
    let country = ctx.auth.country()?;
    let settings = crate::domain::settings::Settings {
        country,
        user_name: ctx.auth.user_name()?,
    };
    ctx.output.settings(settings)
}

fn user(ctx: &AppContext, name: Option<String>) -> Result<()> {
    if let Some(name) = name {
        ctx.auth.set_user_name(Some(name))?;
    }
    let user_name = ctx.auth.user_name()?;
    let settings = crate::domain::settings::Settings {
        country: ctx.auth.country()?,
        user_name,
    };
    ctx.output.settings(settings)
}
