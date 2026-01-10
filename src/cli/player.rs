//! Player command handlers.
use clap::{Subcommand, ValueEnum};

use crate::AppContext;
use crate::cli::now_playing;
use crate::error::Result;

#[derive(Subcommand, Debug)]
pub enum PlayerCommand {
    Play,
    Pause,
    Toggle,
    Next,
    Prev,
    Status,
    Shuffle {
        #[arg(value_enum, help = "Shuffle state")]
        state: ShuffleStateArg,
    },
    Repeat {
        #[arg(value_enum, help = "Repeat state")]
        state: RepeatStateArg,
    },
    Volume {
        #[arg(value_name = "PERCENT", help = "Volume level (0-100). Omit to show current volume")]
        percent: Option<u32>,
    },
}

pub fn handle(command: PlayerCommand, ctx: &AppContext) -> Result<()> {
    let playback = ctx.spotify()?.playback();

    match command {
        PlayerCommand::Play => {
            playback.play()?;
            now_playing::show_with_delay(ctx, 100)
        }
        PlayerCommand::Pause => {
            playback.pause()?;
            ctx.output.action("player_pause", "Paused")
        }
        PlayerCommand::Toggle => {
            let status = playback.status()?;
            if status.is_playing {
                playback.pause()?;
                return ctx.output.action("player_pause", "Paused");
            }
            playback.play()?;
            now_playing::show_with_delay(ctx, 100)
        }
        PlayerCommand::Next => {
            playback.next()?;
            now_playing::show_with_delay(ctx, 100)
        }
        PlayerCommand::Prev => {
            playback.previous()?;
            now_playing::show_with_delay(ctx, 100)
        }
        PlayerCommand::Status => {
            let status = playback.status()?;
            ctx.output.player_status(status)
        }
        PlayerCommand::Shuffle { state } => {
            let enabled = matches!(state, ShuffleStateArg::On);
            playback.shuffle(enabled)?;
            let message = format!("Shuffle: {}", state.as_str());
            ctx.output.action("player_shuffle", &message)
        }
        PlayerCommand::Repeat { state } => {
            playback.repeat(state.as_str())?;
            let message = format!("Repeat: {}", state.as_str());
            ctx.output.action("player_repeat", &message)
        }
        PlayerCommand::Volume { percent } => match percent {
            Some(level) => {
                if level > 100 {
                    anyhow::bail!("volume must be between 0 and 100");
                }
                playback.set_volume(level)?;
                let message = format!("Volume: {}%", level);
                ctx.output.action("player_volume", &message)
            }
            None => {
                let status = playback.status()?;
                let volume = status
                    .device
                    .and_then(|d| d.volume_percent)
                    .map(|v| format!("{}%", v))
                    .unwrap_or_else(|| "unknown".to_string());
                let message = format!("Volume: {}", volume);
                ctx.output.action("player_volume", &message)
            }
        },
    }
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub(crate) enum ShuffleStateArg {
    On,
    Off,
}

impl ShuffleStateArg {
    fn as_str(&self) -> &'static str {
        match self {
            ShuffleStateArg::On => "on",
            ShuffleStateArg::Off => "off",
        }
    }
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub(crate) enum RepeatStateArg {
    Off,
    Track,
    Context,
}

impl RepeatStateArg {
    fn as_str(&self) -> &'static str {
        match self {
            RepeatStateArg::Off => "off",
            RepeatStateArg::Track => "track",
            RepeatStateArg::Context => "context",
        }
    }
}
