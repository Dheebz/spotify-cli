//! Pin command handlers.
use clap::Subcommand;

use crate::AppContext;
use crate::error::Result;

#[derive(Subcommand, Debug)]
pub enum PinCommand {
    Add { name: String, url: String },
    Remove { name: String },
    List,
}

pub fn handle(command: PinCommand, ctx: &AppContext) -> Result<()> {
    match command {
        PinCommand::Add { name, url } => add(ctx, name, url),
        PinCommand::Remove { name } => remove(ctx, &name),
        PinCommand::List => list(ctx),
    }
}

fn add(ctx: &AppContext, name: String, url: String) -> Result<()> {
    ctx.cache.pin_store().add(name.clone(), url.clone())?;
    let message = format!("Pinned: {} -> {}", name, url);
    ctx.output.action("pin_add", &message)
}

fn remove(ctx: &AppContext, name: &str) -> Result<()> {
    let removed = ctx.cache.pin_store().remove(name)?;
    let message = if removed {
        format!("Unpinned: {}", name)
    } else {
        format!("No pin found: {}", name)
    };
    ctx.output.action("pin_remove", &message)
}

fn list(ctx: &AppContext) -> Result<()> {
    let pins = ctx.cache.pin_store().load()?;
    ctx.output.pin_list(pins.items)
}
