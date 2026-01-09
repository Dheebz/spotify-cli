//! Queue command handlers.
use clap::Args;

use crate::AppContext;
use crate::error::Result;

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 100;

#[derive(Args, Debug)]
pub struct QueueCommand {
    #[arg(long, value_name = "N", default_value_t = 10)]
    limit: u32,
}

pub fn handle(command: QueueCommand, ctx: &AppContext) -> Result<()> {
    let limit = clamp_limit(command.limit);
    let state = ctx.spotify()?.playback().queue(limit)?;
    let mut items = Vec::new();
    let now_playing_id = state.now_playing.as_ref().map(|track| track.id.clone());

    if let Some(track) = state.now_playing {
        items.push(track);
    }
    for track in state.queue {
        if items.len() >= limit as usize {
            break;
        }
        items.push(track);
    }

    ctx.output.queue(now_playing_id.as_deref(), items)
}

fn clamp_limit(limit: u32) -> u32 {
    if limit == 0 {
        return DEFAULT_LIMIT;
    }
    limit.min(MAX_LIMIT)
}
