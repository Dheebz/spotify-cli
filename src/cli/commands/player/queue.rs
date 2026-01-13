//! Queue commands: list, add, recent

use crate::endpoints::player::{
    add_item_to_playback_queue, get_playback_state, get_recently_played_tracks, get_users_queue,
};
use crate::io::output::{ErrorKind, Response};

use crate::cli::commands::with_client;

pub async fn player_queue_list() -> Response {
    with_client(|client| async move {
        match get_users_queue::get_users_queue(&client).await {
            Ok(Some(payload)) => Response::success_with_payload(200, "Current queue", payload),
            Ok(None) => Response::success_with_payload(
                200,
                "Queue is empty",
                serde_json::json!({ "queue": [] }),
            ),
            Err(e) => Response::from_http_error(&e, "Failed to get queue"),
        }
    })
    .await
}

pub async fn player_queue_add(uri: Option<&str>, now_playing: bool) -> Response {
    // Validate input
    if uri.is_none() && !now_playing {
        return Response::err(400, "Provide a URI or use --now-playing", ErrorKind::Validation);
    }

    let explicit_uri = uri.map(|s| s.to_string());

    with_client(|client| async move {
        let mut uris_to_add: Vec<String> = Vec::new();

        // Add explicit URI if provided
        if let Some(uri) = explicit_uri {
            uris_to_add.push(uri);
        }

        // Add now playing track if requested
        if now_playing {
            match get_playback_state::get_playback_state(&client).await {
                Ok(Some(state)) => {
                    if let Some(uri) = state
                        .get("item")
                        .and_then(|i| i.get("uri"))
                        .and_then(|v| v.as_str())
                    {
                        uris_to_add.push(uri.to_string());
                    } else {
                        return Response::err(404, "Nothing currently playing", ErrorKind::Player);
                    }
                }
                Ok(None) => return Response::err(404, "Nothing currently playing", ErrorKind::Player),
                Err(e) => return Response::from_http_error(&e, "Failed to get playback state"),
            }
        }

        // Add each URI to queue
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
            Ok(Some(payload)) => Response::success_with_payload(200, "Recently played", payload),
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
