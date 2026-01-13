/// Add one or more items to a user's playlist.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/add-tracks-to-playlist
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

pub async fn add_items_to_playlist(
    client: &SpotifyApi,
    playlist_id: &str,
    uris: &[String],
    position: Option<u32>,
) -> Result<Option<Value>, HttpError> {
    let mut body = serde_json::json!({
        "uris": uris
    });

    if let Some(pos) = position {
        body["position"] = serde_json::Value::Number(pos.into());
    }

    client.post_json(&Endpoint::PlaylistTracks { id: playlist_id }.path(), &body).await
}
