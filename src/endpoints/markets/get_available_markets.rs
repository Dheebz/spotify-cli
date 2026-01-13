use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get available markets
pub async fn get_available_markets(
    client: &SpotifyApi,
) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::Markets.path()).await
}
