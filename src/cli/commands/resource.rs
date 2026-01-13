use crate::endpoints::albums::get_album;
use crate::endpoints::artists::{get_artist, get_artist_top_tracks};
use crate::endpoints::player::get_playback_state;
use crate::endpoints::tracks::get_track;
use crate::io::output::{ErrorKind, Response};

use super::{extract_id, with_client};

/// Get track info - defaults to now playing if no ID provided
pub async fn info_track(id: Option<&str>, id_only: bool) -> Response {
    with_client(|client| async move {
        // If ID provided, use it directly
        let track_id = if let Some(id) = id {
            extract_id(id)
        } else {
            // Get from now playing
            match get_playback_state::get_playback_state(&client).await {
                Ok(Some(state)) => {
                    if let Some(item) = state.get("item") {
                        if let Some(id) = item.get("id").and_then(|v| v.as_str()) {
                            id.to_string()
                        } else {
                            return Response::err(404, "No track ID in playback state", ErrorKind::Player);
                        }
                    } else {
                        return Response::err(404, "Nothing currently playing", ErrorKind::Player);
                    }
                }
                Ok(None) => return Response::err(404, "Nothing currently playing", ErrorKind::Player),
                Err(e) => return Response::from_http_error(&e, "Failed to get playback state"),
            }
        };

        // If --id-only, just return the ID
        if id_only {
            return Response::success(200, &track_id);
        }

        match get_track::get_track(&client, &track_id).await {
            Ok(Some(payload)) => Response::success_with_payload(200, "Track details", payload),
            Ok(None) => Response::err(404, "Track not found", ErrorKind::NotFound),
            Err(e) => Response::from_http_error(&e, "Failed to get track"),
        }
    }).await
}

/// Get album info - defaults to now playing album if no ID provided
pub async fn info_album(id: Option<&str>, id_only: bool) -> Response {
    with_client(|client| async move {
        // If ID provided, use it directly
        let album_id = if let Some(id) = id {
            extract_id(id)
        } else {
            // Get from now playing
            match get_playback_state::get_playback_state(&client).await {
                Ok(Some(state)) => {
                    if let Some(item) = state.get("item") {
                        if let Some(album) = item.get("album") {
                            if let Some(id) = album.get("id").and_then(|v| v.as_str()) {
                                id.to_string()
                            } else {
                                return Response::err(404, "No album ID in playback state", ErrorKind::Player);
                            }
                        } else {
                            return Response::err(404, "Current track has no album info", ErrorKind::Player);
                        }
                    } else {
                        return Response::err(404, "Nothing currently playing", ErrorKind::Player);
                    }
                }
                Ok(None) => return Response::err(404, "Nothing currently playing", ErrorKind::Player),
                Err(e) => return Response::from_http_error(&e, "Failed to get playback state"),
            }
        };

        // If --id-only, just return the ID
        if id_only {
            return Response::success(200, &album_id);
        }

        match get_album::get_album(&client, &album_id).await {
            Ok(Some(payload)) => Response::success_with_payload(200, "Album details", payload),
            Ok(None) => Response::err(404, "Album not found", ErrorKind::NotFound),
            Err(e) => Response::from_http_error(&e, "Failed to get album"),
        }
    }).await
}

/// Get artist info - defaults to now playing artist if no ID provided
pub async fn info_artist(id: Option<&str>, id_only: bool, top_tracks: bool, market: &str) -> Response {
    let market = market.to_string();

    with_client(|client| async move {
        // If ID provided, use it directly
        let artist_id = if let Some(id) = id {
            extract_id(id)
        } else {
            // Get from now playing (first/primary artist)
            match get_playback_state::get_playback_state(&client).await {
                Ok(Some(state)) => {
                    if let Some(item) = state.get("item") {
                        if let Some(artists) = item.get("artists").and_then(|v| v.as_array()) {
                            if let Some(first_artist) = artists.first() {
                                if let Some(id) = first_artist.get("id").and_then(|v| v.as_str()) {
                                    id.to_string()
                                } else {
                                    return Response::err(404, "No artist ID in playback state", ErrorKind::Player);
                                }
                            } else {
                                return Response::err(404, "No artists on current track", ErrorKind::Player);
                            }
                        } else {
                            return Response::err(404, "No artists on current track", ErrorKind::Player);
                        }
                    } else {
                        return Response::err(404, "Nothing currently playing", ErrorKind::Player);
                    }
                }
                Ok(None) => return Response::err(404, "Nothing currently playing", ErrorKind::Player),
                Err(e) => return Response::from_http_error(&e, "Failed to get playback state"),
            }
        };

        // If --id-only, just return the ID
        if id_only {
            return Response::success(200, &artist_id);
        }

        if top_tracks {
            match get_artist_top_tracks::get_artist_top_tracks(&client, &artist_id, Some(&market)).await {
                Ok(Some(payload)) => Response::success_with_payload(200, "Top tracks", payload),
                Ok(None) => Response::success_with_payload(200, "No top tracks", serde_json::json!({ "tracks": [] })),
                Err(e) => Response::from_http_error(&e, "Failed to get top tracks"),
            }
        } else {
            match get_artist::get_artist(&client, &artist_id).await {
                Ok(Some(payload)) => Response::success_with_payload(200, "Artist details", payload),
                Ok(None) => Response::err(404, "Artist not found", ErrorKind::NotFound),
                Err(e) => Response::from_http_error(&e, "Failed to get artist"),
            }
        }
    }).await
}
