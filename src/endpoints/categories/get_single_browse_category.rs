use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get a single browse category
pub async fn get_single_browse_category(
    client: &SpotifyApi,
    category_id: &str,
) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::Category { id: category_id }.path()).await
}
