//! Markets command handlers

use crate::endpoints::markets::get_available_markets;
use crate::io::output::Response;

use super::with_client;

/// List available markets
pub async fn markets_list() -> Response {
    with_client(|client| async move {
        match get_available_markets::get_available_markets(&client).await {
            Ok(Some(payload)) => Response::success_with_payload(200, "Available markets", payload),
            Ok(None) => Response::success_with_payload(
                200,
                "No markets found",
                serde_json::json!({ "markets": [] }),
            ),
            Err(e) => Response::from_http_error(&e, "Failed to get available markets"),
        }
    })
    .await
}
