/// Remove one or more items from a playlist.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/remove-tracks-playlist
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

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
