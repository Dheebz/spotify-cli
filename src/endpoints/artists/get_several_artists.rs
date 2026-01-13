use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get multiple artists by IDs
pub async fn get_several_artists(
    client: &SpotifyApi,
    artist_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = artist_ids.join(",");
    client.get(&Endpoint::Artists { ids: &ids }.path()).await
}
