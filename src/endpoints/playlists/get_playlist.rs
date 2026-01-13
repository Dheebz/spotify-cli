/// Get a playlist owned by a Spotify user.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/get-playlist
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

pub async fn get_playlist(
    client: &SpotifyApi,
    playlist_id: &str,
) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::Playlist { id: playlist_id }.path()).await
}
