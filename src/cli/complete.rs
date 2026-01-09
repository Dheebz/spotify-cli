//! Hidden completion data endpoints for shell scripts.
use clap::Subcommand;

use crate::error::Result;
use crate::AppContext;

#[derive(Subcommand, Debug)]
pub enum CompleteCommand {
    Playlist,
    Pin,
    Device,
}

pub fn handle(command: CompleteCommand, ctx: &AppContext) -> Result<()> {
    match command {
        CompleteCommand::Playlist => playlist(ctx),
        CompleteCommand::Pin => pin(ctx),
        CompleteCommand::Device => device(ctx),
    }
}

fn playlist(ctx: &AppContext) -> Result<()> {
    let snapshot = ctx.cache.playlist_cache().load()?;
    let Some(snapshot) = snapshot else {
        return Ok(());
    };
    for playlist in snapshot.items {
        println!("{}", playlist.name);
    }
    Ok(())
}

fn pin(ctx: &AppContext) -> Result<()> {
    let pins = ctx.cache.pin_store().load()?;
    for pin in pins.items {
        println!("{}", pin.name);
    }
    Ok(())
}

fn device(ctx: &AppContext) -> Result<()> {
    let snapshot = ctx.cache.device_cache().load()?;
    let Some(snapshot) = snapshot else {
        return Ok(());
    };
    for device in snapshot.items {
        println!("{}", device.name);
    }
    Ok(())
}
