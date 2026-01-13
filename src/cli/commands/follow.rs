//! Follow command handlers

use crate::endpoints::user::{
    check_if_user_follows_artist_or_users, follow_artists_or_users, get_followed_artists,
    unfollow_artists_or_users,
};
use crate::io::output::Response;
use crate::types::FollowedArtistsResponse;

use super::with_client;

/// Follow artists
pub async fn follow_artist(ids: &[String], dry_run: bool) -> Response {
    let ids = ids.to_vec();
    let count = ids.len();

    if dry_run {
        return Response::success_with_payload(
            200,
            format!("[DRY RUN] Would follow {} artist(s)", count),
            serde_json::json!({
                "dry_run": true,
                "action": "follow",
                "type": "artist",
                "ids": ids
            }),
        );
    }

    with_client(|client| async move {
        match follow_artists_or_users::follow_artists_or_users(&client, "artist", &ids).await {
            Ok(_) => Response::success(200, format!("Followed {} artist(s)", count)),
            Err(e) => Response::from_http_error(&e, "Failed to follow artists"),
        }
    })
    .await
}

/// Follow users
pub async fn follow_user(ids: &[String], dry_run: bool) -> Response {
    let ids = ids.to_vec();
    let count = ids.len();

    if dry_run {
        return Response::success_with_payload(
            200,
            format!("[DRY RUN] Would follow {} user(s)", count),
            serde_json::json!({
                "dry_run": true,
                "action": "follow",
                "type": "user",
                "ids": ids
            }),
        );
    }

    with_client(|client| async move {
        match follow_artists_or_users::follow_artists_or_users(&client, "user", &ids).await {
            Ok(_) => Response::success(200, format!("Followed {} user(s)", count)),
            Err(e) => Response::from_http_error(&e, "Failed to follow users"),
        }
    })
    .await
}

/// Unfollow artists
pub async fn unfollow_artist(ids: &[String], dry_run: bool) -> Response {
    let ids = ids.to_vec();
    let count = ids.len();

    if dry_run {
        return Response::success_with_payload(
            200,
            format!("[DRY RUN] Would unfollow {} artist(s)", count),
            serde_json::json!({
                "dry_run": true,
                "action": "unfollow",
                "type": "artist",
                "ids": ids
            }),
        );
    }

    with_client(|client| async move {
        match unfollow_artists_or_users::unfollow_artists_or_users(&client, "artist", &ids).await {
            Ok(_) => Response::success(200, format!("Unfollowed {} artist(s)", count)),
            Err(e) => Response::from_http_error(&e, "Failed to unfollow artists"),
        }
    })
    .await
}

/// Unfollow users
pub async fn unfollow_user(ids: &[String], dry_run: bool) -> Response {
    let ids = ids.to_vec();
    let count = ids.len();

    if dry_run {
        return Response::success_with_payload(
            200,
            format!("[DRY RUN] Would unfollow {} user(s)", count),
            serde_json::json!({
                "dry_run": true,
                "action": "unfollow",
                "type": "user",
                "ids": ids
            }),
        );
    }

    with_client(|client| async move {
        match unfollow_artists_or_users::unfollow_artists_or_users(&client, "user", &ids).await {
            Ok(_) => Response::success(200, format!("Unfollowed {} user(s)", count)),
            Err(e) => Response::from_http_error(&e, "Failed to unfollow users"),
        }
    })
    .await
}

/// List followed artists
pub async fn follow_list(limit: u8) -> Response {
    with_client(|client| async move {
        match get_followed_artists::get_followed_artists(&client, Some(limit)).await {
            Ok(Some(payload)) => {
                match serde_json::from_value::<FollowedArtistsResponse>(payload.clone()) {
                    Ok(resp) => {
                        let count = resp.artists.items.len();
                        let total = resp.artists.total.unwrap_or(count as u32);
                        Response::success_with_payload(
                            200,
                            format!("Following {} artists (showing {})", total, count),
                            payload,
                        )
                    }
                    Err(_) => Response::success_with_payload(200, "Followed artists", payload),
                }
            }
            Ok(None) => Response::success_with_payload(
                200,
                "Not following any artists",
                serde_json::json!({ "artists": { "items": [] } }),
            ),
            Err(e) => Response::from_http_error(&e, "Failed to get followed artists"),
        }
    })
    .await
}

/// Check if following artists
pub async fn follow_check_artist(ids: &[String]) -> Response {
    let ids = ids.to_vec();

    with_client(|client| async move {
        match check_if_user_follows_artist_or_users::check_if_user_follows_artist_or_users(
            &client, "artist", &ids,
        )
        .await
        {
            Ok(Some(payload)) => Response::success_with_payload(200, "Follow check results", payload),
            Ok(None) => Response::success_with_payload(200, "Follow check results", serde_json::json!([])),
            Err(e) => Response::from_http_error(&e, "Failed to check follow status"),
        }
    })
    .await
}

/// Check if following users
pub async fn follow_check_user(ids: &[String]) -> Response {
    let ids = ids.to_vec();

    with_client(|client| async move {
        match check_if_user_follows_artist_or_users::check_if_user_follows_artist_or_users(
            &client, "user", &ids,
        )
        .await
        {
            Ok(Some(payload)) => Response::success_with_payload(200, "Follow check results", payload),
            Ok(None) => Response::success_with_payload(200, "Follow check results", serde_json::json!([])),
            Err(e) => Response::from_http_error(&e, "Failed to check follow status"),
        }
    })
    .await
}
