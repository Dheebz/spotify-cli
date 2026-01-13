/// Get tracks from the current user's recently played tracks.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/get-recently-played
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

pub async fn get_recently_played_tracks(
    client: &SpotifyApi,
) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::PlayerRecentlyPlayed.path()).await
}
