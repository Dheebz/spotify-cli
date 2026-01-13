use crate::endpoints::categories::{get_category_playlists, get_several_browse_categories, get_single_browse_category};
use crate::io::output::{ErrorKind, Response};

use super::with_client;

pub async fn category_list(limit: u8, offset: u32) -> Response {
    with_client(|client| async move {
        match get_several_browse_categories::get_several_browse_categories(&client, Some(limit), Some(offset)).await {
            Ok(Some(payload)) => Response::success_with_payload(200, "Browse categories", payload),
            Ok(None) => Response::success_with_payload(
                200,
                "No categories found",
                serde_json::json!({ "categories": { "items": [] } }),
            ),
            Err(e) => Response::from_http_error(&e, "Failed to get categories"),
        }
    }).await
}

pub async fn category_get(category_id: &str) -> Response {
    let category_id = category_id.to_string();

    with_client(|client| async move {
        match get_single_browse_category::get_single_browse_category(&client, &category_id).await {
            Ok(Some(payload)) => Response::success_with_payload(200, "Category details", payload),
            Ok(None) => Response::err(404, "Category not found", ErrorKind::NotFound),
            Err(e) => Response::from_http_error(&e, "Failed to get category"),
        }
    }).await
}

pub async fn category_playlists(category_id: &str, limit: u8, offset: u32) -> Response {
    let category_id = category_id.to_string();

    with_client(|client| async move {
        match get_category_playlists::get_category_playlists(&client, &category_id, Some(limit), Some(offset)).await {
            Ok(Some(payload)) => Response::success_with_payload(200, format!("Playlists for {}", category_id), payload),
            Ok(None) => Response::success_with_payload(
                200,
                "No playlists found",
                serde_json::json!({ "playlists": { "items": [] } }),
            ),
            Err(e) => Response::from_http_error(&e, "Failed to get category playlists"),
        }
    }).await
}
