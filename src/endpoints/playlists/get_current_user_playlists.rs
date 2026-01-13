/// Get a list of the current user's playlists.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/get-a-list-of-current-users-playlists
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

pub async fn get_current_user_playlists(
    client: &SpotifyApi,
    limit: Option<u8>,
    offset: Option<u32>,
) -> Result<Option<Value>, HttpError> {
    let limit = limit.unwrap_or(20).min(50);
    let offset = offset.unwrap_or(0);

    client.get(&Endpoint::CurrentUserPlaylists { limit, offset }.path()).await
}
