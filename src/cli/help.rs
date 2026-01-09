//! Help command handler.
use clap::Args;

use crate::AppContext;
use crate::error::Result;

#[derive(Args, Debug)]
pub struct HelpCommand;

pub fn handle(_command: HelpCommand, ctx: &AppContext) -> Result<()> {
    ctx.output.help()
}
