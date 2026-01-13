use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Follow a playlist
pub async fn follow_playlist(
    client: &SpotifyApi,
    playlist_id: &str,
    public: Option<bool>,
) -> Result<Option<Value>, HttpError> {
    let body = serde_json::json!({
        "public": public.unwrap_or(true)
    });

    client.put_json(&Endpoint::PlaylistFollowers { id: playlist_id }.path(), &body).await
}
