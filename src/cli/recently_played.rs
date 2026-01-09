//! Recently played command handlers.
use clap::Args;

use crate::AppContext;
use crate::error::Result;

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 100;

#[derive(Args, Debug)]
pub struct RecentlyPlayedCommand {
    #[arg(long, value_name = "N", default_value_t = 10)]
    limit: u32,
}

pub fn handle(command: RecentlyPlayedCommand, ctx: &AppContext) -> Result<()> {
    let limit = clamp_limit(command.limit);
    let status = ctx.spotify()?.playback().status()?;
    let now_playing = status.track.map(map_track);
    let mut items = ctx.spotify()?.search().recently_played(limit)?;
    if let Some(now_playing) = now_playing {
        let now_id = now_playing.id.clone();
        items.retain(|item| item.id != now_id);
        items.insert(0, now_playing);
    }
    let now_playing_id = items.first().map(|item| item.id.clone());
    ctx.output.recently_played(now_playing_id.as_deref(), items)
}

fn clamp_limit(limit: u32) -> u32 {
    if limit == 0 {
        return DEFAULT_LIMIT;
    }
    limit.min(MAX_LIMIT)
}

fn map_track(track: crate::domain::track::Track) -> crate::domain::search::SearchItem {
    let id = track.id;
    crate::domain::search::SearchItem {
        id: id.clone(),
        name: track.name,
        uri: format!("spotify:track:{}", id),
        kind: crate::domain::search::SearchType::Track,
        artists: track.artists,
        album: track.album,
        duration_ms: track.duration_ms,
        owner: None,
        score: None,
    }
}
