//! Now playing command handlers.
use clap::{Args, Subcommand};

use crate::AppContext;
use crate::cli::playlist;
use crate::error::Result;

#[derive(Args, Debug)]
pub struct NowPlayingCommand {
    #[arg(
        long,
        value_name = "MS",
        default_value_t = 0,
        help = "Delay before refresh"
    )]
    delay_ms: u64,
    #[command(subcommand)]
    action: Option<NowPlayingAction>,
}

#[derive(Subcommand, Debug)]
enum NowPlayingAction {
    Like,
    #[command(name = "addto")]
    AddTo {
        #[arg(value_name = "QUERY")]
        query: Option<String>,
        #[arg(long, help = "Use market from token")]
        user: bool,
        #[arg(long, help = "Pick a specific result (1-based)")]
        pick: Option<usize>,
        #[arg(long, help = "Use the last cached search results")]
        last: bool,
    },
}

pub fn handle(command: NowPlayingCommand, ctx: &AppContext) -> Result<()> {
    match command.action {
        None => show_detailed_with_delay(ctx, command.delay_ms),
        Some(NowPlayingAction::Like) => like(ctx),
        Some(NowPlayingAction::AddTo {
            query,
            user,
            pick,
            last,
        }) => playlist::add_to(ctx, query.as_deref(), user, pick, last),
    }
}

pub fn show_with_delay(ctx: &AppContext, delay_ms: u64) -> Result<()> {
    if delay_ms > 0 {
        std::thread::sleep(std::time::Duration::from_millis(delay_ms));
    }
    let status = ctx.spotify()?.playback().status()?;
    ctx.output.now_playing(status)
}

pub fn show_detailed_with_delay(ctx: &AppContext, delay_ms: u64) -> Result<()> {
    if delay_ms > 0 {
        std::thread::sleep(std::time::Duration::from_millis(delay_ms));
    }
    let status = ctx.spotify()?.playback().status()?;
    ctx.output.player_status(status)
}

fn like(ctx: &AppContext) -> Result<()> {
    let status = ctx.spotify()?.playback().status()?;
    let Some(track) = status.track else {
        anyhow::bail!("no track is currently playing");
    };

    ctx.spotify()?.track().like(&track.id)?;
    let message = format!("Liked: {}", format_track(&track));
    ctx.output.action("track_like", &message)
}

fn format_track(track: &crate::domain::track::Track) -> String {
    if track.artists.is_empty() {
        track.name.clone()
    } else {
        format!("{} - {}", track.name, track.artists.join(", "))
    }
}
