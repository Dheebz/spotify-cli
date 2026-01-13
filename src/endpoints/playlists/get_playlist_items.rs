use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get playlist items (tracks)
pub async fn get_playlist_items(
    client: &SpotifyApi,
    playlist_id: &str,
    limit: Option<u8>,
    offset: Option<u32>,
) -> Result<Option<Value>, HttpError> {
    let limit = limit.unwrap_or(20).min(50);
    let offset = offset.unwrap_or(0);
    client.get(&Endpoint::PlaylistItems { id: playlist_id, limit, offset }.path()).await
}
