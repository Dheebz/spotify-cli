//! Cache status output formatting.
use serde::Serialize;

use crate::domain::cache::CacheStatus;
use crate::error::Result;

pub fn status_human(status: CacheStatus) -> Result<()> {
    println!(
        "cache_root={} devices={} playlists={}",
        status.root, status.device_count, status.playlist_count
    );
    Ok(())
}

#[derive(Serialize)]
struct CacheStatusPayload {
    root: String,
    device_count: usize,
    playlist_count: usize,
}

pub fn status_json(status: CacheStatus) -> Result<()> {
    let payload = cache_status_payload(status);
    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

fn cache_status_payload(status: CacheStatus) -> CacheStatusPayload {
    CacheStatusPayload {
        root: status.root,
        device_count: status.device_count,
        playlist_count: status.playlist_count,
    }
}

#[cfg(test)]
mod tests {
    use super::cache_status_payload;
    use crate::domain::cache::CacheStatus;

    #[test]
    fn cache_status_payload_shape() {
        let payload = cache_status_payload(CacheStatus {
            root: "/tmp".to_string(),
            device_count: 1,
            playlist_count: 2,
        });
        assert_eq!(payload.device_count, 1);
        assert_eq!(payload.playlist_count, 2);
    }
}
