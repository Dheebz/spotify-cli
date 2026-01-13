use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Reorder items in a playlist
/// range_start: position of first item to move
/// insert_before: position where items should be inserted
/// range_length: number of items to move (default 1)
pub async fn reorder_playlist_items(
    client: &SpotifyApi,
    playlist_id: &str,
    range_start: u32,
    insert_before: u32,
    range_length: Option<u32>,
) -> Result<Option<Value>, HttpError> {
    let mut body = serde_json::json!({
        "range_start": range_start,
        "insert_before": insert_before,
    });

    if let Some(len) = range_length {
        body["range_length"] = serde_json::Value::Number(len.into());
    }

    client.put_json(&Endpoint::PlaylistTracks { id: playlist_id }.path(), &body).await
}
