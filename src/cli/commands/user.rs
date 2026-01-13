use crate::endpoints::user::{get_current_user, get_users_top_items};
use crate::io::output::{ErrorKind, Response};

use super::with_client;

pub async fn user_profile() -> Response {
    with_client(|client| async move {
        match get_current_user::get_current_user(&client).await {
            Ok(Some(payload)) => Response::success_with_payload(200, "User profile", payload),
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
                Response::success_with_payload(
                    200,
                    format!("Top {} ({})", item_type, range_desc),
                    payload,
                )
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
