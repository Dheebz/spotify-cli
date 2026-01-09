//! Sync command handler.
use clap::Args;

use crate::AppContext;
use crate::cache::devices::CacheSnapshot as DeviceSnapshot;
use crate::cache::playlists::CacheSnapshot as PlaylistSnapshot;
use crate::error::Result;

#[derive(Args, Debug)]
pub struct SyncCommand;

pub fn handle(_command: SyncCommand, ctx: &AppContext) -> Result<()> {
    ctx.auth.ensure_user_name()?;
    let devices = ctx.spotify()?.devices().list()?;
    let playlists = ctx.spotify()?.playlists().list_all()?;
    let updated_at = unix_time();

    let device_snapshot = DeviceSnapshot {
        updated_at,
        items: devices,
    };
    let playlist_snapshot = PlaylistSnapshot {
        updated_at,
        items: playlists,
    };

    ctx.cache.device_cache().save(&device_snapshot)?;
    ctx.cache.playlist_cache().save(&playlist_snapshot)?;
    let message = format!(
        "Synced: devices={} playlists={}",
        device_snapshot.items.len(),
        playlist_snapshot.items.len()
    );
    ctx.output.action("sync", &message)
}

fn unix_time() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_secs()
}
