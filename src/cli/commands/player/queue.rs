//! Queue commands: list, add, recent

use crate::endpoints::player::{
    add_item_to_playback_queue, get_recently_played_tracks, get_users_queue,
};
use crate::io::output::{ErrorKind, Response};
use crate::types::{QueueResponse, RecentlyPlayedResponse};

use crate::cli::commands::{now_playing, with_client};

pub async fn player_queue_list() -> Response {
    with_client(|client| async move {
        match get_users_queue::get_users_queue(&client).await {
            Ok(Some(payload)) => {
                // Validate response structure by deserializing to typed struct
                match serde_json::from_value::<QueueResponse>(payload.clone()) {
                    Ok(_) => Response::success_with_payload(200, "Current queue", payload),
                    Err(_) => Response::success_with_payload(200, "Current queue", payload),
                }
            }
            Ok(None) => Response::success_with_payload(
                200,
                "Queue is empty",
                serde_json::json!({ "queue": [], "currently_playing": null }),
            ),
            Err(e) => Response::from_http_error(&e, "Failed to get queue"),
        }
    })
    .await
}

pub async fn player_queue_add(uri: Option<&str>, now_playing_flag: bool) -> Response {
    if uri.is_none() && !now_playing_flag {
        return Response::err(400, "Provide a URI or use --now-playing", ErrorKind::Validation);
    }

    let explicit_uri = uri.map(|s| s.to_string());

    with_client(|client| async move {
        let mut uris_to_add: Vec<String> = Vec::new();

        if let Some(uri) = explicit_uri {
            uris_to_add.push(uri);
        }

        if now_playing_flag {
            match now_playing::get_track_uri(&client).await {
                Ok(uri) => uris_to_add.push(uri),
                Err(e) => return e,
            }
        }

        for uri in &uris_to_add {
            if let Err(e) = add_item_to_playback_queue::add_item_to_playback_queue(&client, uri).await {
                return Response::from_http_error(&e, "Failed to add to queue");
            }
        }

        Response::success(204, "Added to queue")
    })
    .await
}

pub async fn player_recent() -> Response {
    with_client(|client| async move {
        match get_recently_played_tracks::get_recently_played_tracks(&client).await {
            Ok(Some(payload)) => {
                // Validate response structure by deserializing to typed struct
                match serde_json::from_value::<RecentlyPlayedResponse>(payload.clone()) {
                    Ok(resp) => {
                        let count = resp.items.len();
                        Response::success_with_payload(
                            200,
                            format!("Recently played ({} tracks)", count),
                            payload,
                        )
                    }
                    Err(_) => Response::success_with_payload(200, "Recently played", payload),
                }
            }
            Ok(None) => Response::success_with_payload(
                200,
                "No recent tracks",
                serde_json::json!({ "items": [] }),
            ),
            Err(e) => Response::from_http_error(&e, "Failed to get recent tracks"),
        }
    })
    .await
}
