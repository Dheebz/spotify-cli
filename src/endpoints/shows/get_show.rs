use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get show (podcast) details by ID
pub async fn get_show(
    client: &SpotifyApi,
    show_id: &str,
) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::Show { id: show_id }.path()).await
}
