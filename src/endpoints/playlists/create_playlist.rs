/// Create a playlist for a Spotify user.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/create-playlist
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

pub async fn create_playlist(
    client: &SpotifyApi,
    user_id: &str,
    name: &str,
    description: Option<&str>,
    public: bool,
) -> Result<Option<Value>, HttpError> {
    let mut body = serde_json::json!({
        "name": name,
        "public": public
    });

    if let Some(desc) = description {
        body["description"] = serde_json::Value::String(desc.to_string());
    }

    client.post_json(&Endpoint::UserPlaylists { user_id }.path(), &body).await
}
