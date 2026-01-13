use crate::endpoints::playlists::{
    add_items_to_playlist, change_playlist_details, create_playlist, follow_playlist,
    get_current_user_playlists, get_featured_playlists, get_playlist, get_playlist_cover_image,
    get_users_playlists, remove_items_from_playlist, unfollow_playlist, update_playlist_items,
};
use crate::endpoints::user::get_current_user;
use crate::io::output::{ErrorKind, Response};
use crate::storage::pins::{PinStore, ResourceType};

use super::{extract_id, now_playing, with_client};

/// Resolve a playlist identifier to a Spotify ID
/// Accepts: ID, URL, or pin alias
fn resolve_playlist_id(input: &str) -> Result<String, Response> {
    if let Ok(store) = PinStore::new()
        && let Some(pin) = store.find_by_alias(input)
            && pin.resource_type == ResourceType::Playlist {
                return Ok(pin.id.clone());
            }
    Ok(extract_id(input))
}

pub async fn playlist_list(limit: u8, offset: u32) -> Response {
    with_client(|client| async move {
        match get_current_user_playlists::get_current_user_playlists(&client, Some(limit), Some(offset)).await {
            Ok(Some(payload)) => Response::success_with_payload(200, "Your playlists", payload),
            Ok(None) => Response::success_with_payload(
                200,
                "No playlists",
                serde_json::json!({ "items": [] }),
            ),
            Err(e) => Response::from_http_error(&e, "Failed to get playlists"),
        }
    }).await
}

pub async fn playlist_get(playlist: &str) -> Response {
    let playlist_id = match resolve_playlist_id(playlist) {
        Ok(id) => id,
        Err(e) => return e,
    };

    with_client(|client| async move {
        match get_playlist::get_playlist(&client, &playlist_id).await {
            Ok(Some(payload)) => Response::success_with_payload(200, "Playlist details", payload),
            Ok(None) => Response::err(404, "Playlist not found", ErrorKind::NotFound),
            Err(e) => Response::from_http_error(&e, "Failed to get playlist"),
        }
    }).await
}

pub async fn playlist_create(name: &str, description: Option<&str>, public: bool) -> Response {
    let name = name.to_string();
    let description = description.map(|s| s.to_string());

    with_client(|client| async move {
        let user_id = match get_current_user::get_current_user(&client).await {
            Ok(Some(user)) => match user.get("id").and_then(|v| v.as_str()) {
                Some(id) => id.to_string(),
                None => return Response::err(500, "Could not get user ID", ErrorKind::Api),
            },
            Ok(None) => return Response::err(500, "Could not get user info", ErrorKind::Api),
            Err(e) => return Response::from_http_error(&e, "Failed to get user info"),
        };

        match create_playlist::create_playlist(&client, &user_id, &name, description.as_deref(), public).await {
            Ok(Some(payload)) => Response::success_with_payload(201, "Playlist created", payload),
            Ok(None) => Response::err(500, "Failed to create playlist", ErrorKind::Api),
            Err(e) => Response::from_http_error(&e, "Failed to create playlist"),
        }
    }).await
}

pub async fn playlist_add(playlist: &str, uris: &[String], now_playing_flag: bool, position: Option<u32>, dry_run: bool) -> Response {
    if uris.is_empty() && !now_playing_flag {
        return Response::err(400, "Provide track URIs or use --now-playing", ErrorKind::Validation);
    }

    let playlist_id = match resolve_playlist_id(playlist) {
        Ok(id) => id,
        Err(e) => return e,
    };
    let explicit_uris = uris.to_vec();

    with_client(|client| async move {
        let mut all_uris = explicit_uris;

        if now_playing_flag {
            match now_playing::get_track_uri(&client).await {
                Ok(uri) => all_uris.push(uri),
                Err(e) => return e,
            }
        }

        let uri_count = all_uris.len();

        if dry_run {
            return Response::success_with_payload(
                200,
                format!("[DRY RUN] Would add {} track(s) to playlist {}", uri_count, playlist_id),
                serde_json::json!({
                    "dry_run": true,
                    "action": "add",
                    "playlist_id": playlist_id,
                    "uris": all_uris,
                    "position": position
                }),
            );
        }

        match add_items_to_playlist::add_items_to_playlist(&client, &playlist_id, &all_uris, position).await {
            Ok(Some(payload)) => Response::success_with_payload(201, format!("Added {} track(s)", uri_count), payload),
            Ok(None) => Response::success(201, format!("Added {} track(s)", uri_count)),
            Err(e) => Response::from_http_error(&e, "Failed to add tracks"),
        }
    }).await
}

pub async fn playlist_remove(playlist: &str, uris: &[String], dry_run: bool) -> Response {
    let playlist_id = match resolve_playlist_id(playlist) {
        Ok(id) => id,
        Err(e) => return e,
    };
    let uris = uris.to_vec();
    let uri_count = uris.len();

    if dry_run {
        return Response::success_with_payload(
            200,
            format!("[DRY RUN] Would remove {} track(s) from playlist {}", uri_count, playlist_id),
            serde_json::json!({
                "dry_run": true,
                "action": "remove",
                "playlist_id": playlist_id,
                "uris": uris
            }),
        );
    }

    with_client(|client| async move {
        match remove_items_from_playlist::remove_items_from_playlist(&client, &playlist_id, &uris).await {
            Ok(Some(payload)) => Response::success_with_payload(200, format!("Removed {} track(s)", uri_count), payload),
            Ok(None) => Response::success(200, format!("Removed {} track(s)", uri_count)),
            Err(e) => Response::from_http_error(&e, "Failed to remove tracks"),
        }
    }).await
}

pub async fn playlist_edit(
    playlist: &str,
    name: Option<&str>,
    description: Option<&str>,
    public: Option<bool>,
) -> Response {
    if name.is_none() && description.is_none() && public.is_none() {
        return Response::err(
            400,
            "No changes specified. Use --name, --description, --public, or --private",
            ErrorKind::Validation,
        );
    }

    let playlist_id = match resolve_playlist_id(playlist) {
        Ok(id) => id,
        Err(e) => return e,
    };
    let name = name.map(|s| s.to_string());
    let description = description.map(|s| s.to_string());

    with_client(|client| async move {
        match change_playlist_details::change_playlist_details(
            &client,
            &playlist_id,
            name.as_deref(),
            description.as_deref(),
            public,
        ).await {
            Ok(_) => Response::success(200, "Playlist updated"),
            Err(e) => Response::from_http_error(&e, "Failed to update playlist"),
        }
    }).await
}

pub async fn playlist_reorder(playlist: &str, from: u32, to: u32, count: u32) -> Response {
    let playlist_id = match resolve_playlist_id(playlist) {
        Ok(id) => id,
        Err(e) => return e,
    };

    with_client(|client| async move {
        match update_playlist_items::reorder_playlist_items(&client, &playlist_id, from, to, Some(count)).await {
            Ok(_) => Response::success(200, format!("Moved {} track(s) from position {} to {}", count, from, to)),
            Err(e) => Response::from_http_error(&e, "Failed to reorder tracks"),
        }
    }).await
}

pub async fn playlist_follow(playlist: &str, public: bool) -> Response {
    let playlist_id = extract_id(playlist);

    with_client(|client| async move {
        match follow_playlist::follow_playlist(&client, &playlist_id, Some(public)).await {
            Ok(_) => Response::success(200, "Following playlist"),
            Err(e) => Response::from_http_error(&e, "Failed to follow playlist"),
        }
    }).await
}

pub async fn playlist_unfollow(playlist: &str) -> Response {
    let playlist_id = extract_id(playlist);

    with_client(|client| async move {
        match unfollow_playlist::unfollow_playlist(&client, &playlist_id).await {
            Ok(_) => Response::success(200, "Unfollowed playlist"),
            Err(e) => Response::from_http_error(&e, "Failed to unfollow playlist"),
        }
    }).await
}

pub async fn playlist_duplicate(playlist: &str, new_name: Option<&str>) -> Response {
    let playlist_id = match resolve_playlist_id(playlist) {
        Ok(id) => id,
        Err(e) => return e,
    };
    let new_name = new_name.map(|s| s.to_string());

    with_client(|client| async move {
        // Get the source playlist
        let source = match get_playlist::get_playlist(&client, &playlist_id).await {
            Ok(Some(p)) => p,
            Ok(None) => return Response::err(404, "Playlist not found", ErrorKind::NotFound),
            Err(e) => return Response::from_http_error(&e, "Failed to get playlist"),
        };

        // Get source details
        let source_name = source.get("name").and_then(|v| v.as_str()).unwrap_or("Playlist");
        let default_name = format!("{} (Copy)", source_name);
        let name = new_name.as_deref().unwrap_or(&default_name);
        let description = source.get("description").and_then(|v| v.as_str());

        // Get current user ID
        let user = match get_current_user::get_current_user(&client).await {
            Ok(Some(u)) => u,
            Ok(None) => return Response::err(500, "Failed to get user info", ErrorKind::Api),
            Err(e) => return Response::from_http_error(&e, "Failed to get user info"),
        };
        let user_id = match user.get("id").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return Response::err(500, "Failed to get user ID", ErrorKind::Api),
        };

        // Create new playlist
        let new_playlist = match create_playlist::create_playlist(&client, user_id, name, description, false).await {
            Ok(Some(p)) => p,
            Ok(None) => return Response::err(500, "Failed to create playlist", ErrorKind::Api),
            Err(e) => return Response::from_http_error(&e, "Failed to create playlist"),
        };

        let new_playlist_id = match new_playlist.get("id").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return Response::err(500, "Failed to get new playlist ID", ErrorKind::Api),
        };

        // Get tracks from source and add to new playlist
        if let Some(tracks) = source.get("tracks").and_then(|t| t.get("items")).and_then(|i| i.as_array()) {
            let uris: Vec<String> = tracks
                .iter()
                .filter_map(|item| {
                    item.get("track")
                        .and_then(|t| t.get("uri"))
                        .and_then(|u| u.as_str())
                        .map(|s| s.to_string())
                })
                .collect();

            if !uris.is_empty()
                && let Err(e) = add_items_to_playlist::add_items_to_playlist(&client, new_playlist_id, &uris, None).await {
                    return Response::from_http_error(&e, "Created playlist but failed to copy tracks");
                }
        }

        Response::success_with_payload(200, format!("Duplicated playlist as '{}'", name), new_playlist)
    }).await
}

/// Get featured playlists
pub async fn playlist_featured(limit: u8, offset: u32) -> Response {
    with_client(|client| async move {
        match get_featured_playlists::get_featured_playlists(&client, Some(limit), Some(offset)).await {
            Ok(Some(payload)) => Response::success_with_payload(200, "Featured playlists", payload),
            Ok(None) => Response::success_with_payload(
                200,
                "No featured playlists",
                serde_json::json!({ "playlists": { "items": [] } }),
            ),
            Err(e) => Response::from_http_error(&e, "Failed to get featured playlists"),
        }
    }).await
}

/// Get playlist cover image URL
pub async fn playlist_cover(playlist: &str) -> Response {
    let playlist_id = match resolve_playlist_id(playlist) {
        Ok(id) => id,
        Err(e) => return e,
    };

    with_client(|client| async move {
        match get_playlist_cover_image::get_playlist_cover_image(&client, &playlist_id).await {
            Ok(Some(payload)) => Response::success_with_payload(200, "Playlist cover image", payload),
            Ok(None) => Response::err(404, "No cover image found", ErrorKind::NotFound),
            Err(e) => Response::from_http_error(&e, "Failed to get playlist cover"),
        }
    }).await
}

/// Get another user's playlists
pub async fn playlist_user(user_id: &str) -> Response {
    let user_id = user_id.to_string();

    with_client(|client| async move {
        match get_users_playlists::get_users_playlists(&client, &user_id).await {
            Ok(Some(payload)) => Response::success_with_payload(200, format!("Playlists for user {}", user_id), payload),
            Ok(None) => Response::success_with_payload(
                200,
                "No playlists found",
                serde_json::json!({ "items": [] }),
            ),
            Err(e) => Response::from_http_error(&e, "Failed to get user's playlists"),
        }
    }).await
}
