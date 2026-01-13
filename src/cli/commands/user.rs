use crate::endpoints::user::{get_current_user, get_users_profile, get_users_top_items};
use crate::io::output::{ErrorKind, Response};
use crate::types::{TopArtistsResponse, TopTracksResponse, UserPrivate, UserPublic};

use super::with_client;

pub async fn user_profile() -> Response {
    with_client(|client| async move {
        match get_current_user::get_current_user(&client).await {
            Ok(Some(payload)) => {
                match serde_json::from_value::<UserPrivate>(payload.clone()) {
                    Ok(user) => {
                        let name = user.display_name.as_deref().unwrap_or(&user.id);
                        let product = user.product.as_deref().unwrap_or("free");
                        Response::success_with_payload(200, format!("{} ({})", name, product), payload)
                    }
                    Err(_) => Response::success_with_payload(200, "User profile", payload),
                }
            }
            Ok(None) => Response::err(404, "User not found", ErrorKind::NotFound),
            Err(e) => Response::from_http_error(&e, "Failed to get user profile"),
        }
    })
    .await
}

pub async fn user_top(item_type: &str, range: &str, limit: u8) -> Response {
    let item_type = item_type.to_string();
    let range = range.to_string();

    with_client(|client| async move {
        let time_range = match range.as_str() {
            "short" => "short_term",
            "medium" => "medium_term",
            "long" => "long_term",
            _ => "medium_term",
        };

        match get_users_top_items::get_users_top_items(
            &client,
            &item_type,
            Some(time_range),
            Some(limit),
            None,
        )
        .await
        {
            Ok(Some(payload)) => {
                let range_desc = match range.as_str() {
                    "short" => "4 weeks",
                    "medium" => "6 months",
                    "long" => "all time",
                    _ => "6 months",
                };

                // Try to get count from typed response
                let count = if item_type == "tracks" {
                    serde_json::from_value::<TopTracksResponse>(payload.clone())
                        .ok()
                        .map(|r| r.items.len())
                } else {
                    serde_json::from_value::<TopArtistsResponse>(payload.clone())
                        .ok()
                        .map(|r| r.items.len())
                };

                let msg = match count {
                    Some(n) => format!("Top {} {} ({})", n, item_type, range_desc),
                    None => format!("Top {} ({})", item_type, range_desc),
                };
                Response::success_with_payload(200, msg, payload)
            }
            Ok(None) => Response::success_with_payload(
                200,
                format!("No top {} found", item_type),
                serde_json::json!({ "items": [] }),
            ),
            Err(e) => Response::from_http_error(&e, &format!("Failed to get top {}", item_type)),
        }
    })
    .await
}

/// Get another user's profile
pub async fn user_get(user_id: &str) -> Response {
    let user_id = user_id.to_string();

    with_client(|client| async move {
        match get_users_profile::get_users_profile(&client, &user_id).await {
            Ok(Some(payload)) => {
                match serde_json::from_value::<UserPublic>(payload.clone()) {
                    Ok(user) => {
                        let name = user.display_name.as_deref().unwrap_or(&user.id);
                        Response::success_with_payload(200, name.to_string(), payload)
                    }
                    Err(_) => Response::success_with_payload(200, format!("User {}", user_id), payload),
                }
            }
            Ok(None) => Response::err(404, "User not found", ErrorKind::NotFound),
            Err(e) => Response::from_http_error(&e, "Failed to get user profile"),
        }
    })
    .await
}
