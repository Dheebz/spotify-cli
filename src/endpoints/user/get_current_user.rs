/// Get detailed profile information about the current user.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/get-current-users-profile
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

pub async fn get_current_user(client: &SpotifyApi) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::CurrentUser.path()).await
}
