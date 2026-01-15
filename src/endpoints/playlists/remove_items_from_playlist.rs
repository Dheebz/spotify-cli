/// Remove one or more items from a playlist.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/remove-tracks-playlist
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Remove all instances of the given URIs from a playlist
pub async fn remove_items_from_playlist(
    client: &SpotifyApi,
    playlist_id: &str,
    uris: &[String],
) -> Result<Option<Value>, HttpError> {
    let tracks: Vec<serde_json::Value> = uris
        .iter()
        .map(|uri| serde_json::json!({ "uri": uri }))
        .collect();

    let body = serde_json::json!({
        "tracks": tracks
    });

    client.delete_json(&Endpoint::PlaylistTracks { id: playlist_id }.path(), &body).await
}

/// Remove items at specific positions from a playlist
/// Takes a list of (uri, position) tuples
/// Note: Positions must be provided in descending order to avoid index shifting issues
pub async fn remove_items_at_positions(
    client: &SpotifyApi,
    playlist_id: &str,
    items: &[(String, usize)],
) -> Result<Option<Value>, HttpError> {
    // Sort by position descending - remove from end first to avoid index shifting
    let mut sorted_items: Vec<(String, usize)> = items.to_vec();
    sorted_items.sort_by(|a, b| b.1.cmp(&a.1));

    // Send each item separately to avoid the URI-based removal behavior
    let tracks: Vec<serde_json::Value> = sorted_items
        .iter()
        .map(|(uri, pos)| serde_json::json!({ "uri": uri, "positions": [*pos] }))
        .collect();

    let body = serde_json::json!({
        "tracks": tracks
    });

    client.delete_json(&Endpoint::PlaylistTracks { id: playlist_id }.path(), &body).await
}
