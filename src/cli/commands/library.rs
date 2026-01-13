//! Library (liked songs) command handlers

use crate::endpoints::library::{check_saved_tracks, get_saved_tracks, remove_tracks, save_tracks};
use crate::io::output::{ErrorKind, Response};

use super::{now_playing, with_client};

resource_list!(library_list, get_saved_tracks::get_saved_tracks, "Saved tracks");
resource_check!(library_check, check_saved_tracks::check_saved_tracks);

pub async fn library_save(ids: &[String], now_playing_flag: bool, dry_run: bool) -> Response {
    if ids.is_empty() && !now_playing_flag {
        return Response::err(400, "Provide track IDs or use --now-playing", ErrorKind::Validation);
    }

    let explicit_ids = ids.to_vec();

    with_client(|client| async move {
        let mut all_ids = explicit_ids;

        if now_playing_flag {
            match now_playing::get_track_id(&client).await {
                Ok(id) => all_ids.push(id),
                Err(e) => return e,
            }
        }

        let count = all_ids.len();

        if dry_run {
            return Response::success_with_payload(
                200,
                format!("[DRY RUN] Would save {} track(s) to library", count),
                serde_json::json!({
                    "dry_run": true,
                    "action": "save",
                    "ids": all_ids
                }),
            );
        }

        match save_tracks::save_tracks(&client, &all_ids).await {
            Ok(_) => Response::success(200, format!("Saved {} track(s)", count)),
            Err(e) => Response::from_http_error(&e, "Failed to save tracks"),
        }
    }).await
}

pub async fn library_remove(ids: &[String], dry_run: bool) -> Response {
    if ids.is_empty() {
        return Response::err(400, "Provide track IDs to remove", ErrorKind::Validation);
    }

    let ids = ids.to_vec();
    let count = ids.len();

    if dry_run {
        return Response::success_with_payload(
            200,
            format!("[DRY RUN] Would remove {} track(s) from library", count),
            serde_json::json!({
                "dry_run": true,
                "action": "remove",
                "ids": ids
            }),
        );
    }

    with_client(|client| async move {
        match remove_tracks::remove_tracks(&client, &ids).await {
            Ok(_) => Response::success(200, format!("Removed {} track(s)", count)),
            Err(e) => Response::from_http_error(&e, "Failed to remove tracks"),
        }
    }).await
}
