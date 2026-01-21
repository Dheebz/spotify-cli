use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Start or resume playback
/// If context_uri is provided, plays that context (playlist, album, artist)
/// If uris is provided, plays those specific tracks
pub async fn start_resume_playback(
    client: &SpotifyApi,
    context_uri: Option<&str>,
    uris: Option<&[String]>,
) -> Result<Option<Value>, HttpError> {
    let endpoint = Endpoint::PlayerPlay.path();

    // If no context or URIs, just resume playback
    if context_uri.is_none() && uris.is_none() {
        return client.put(&endpoint).await;
    }

    let mut body = serde_json::json!({});

    if let Some(ctx) = context_uri {
        body["context_uri"] = serde_json::Value::String(ctx.to_string());
    }

    if let Some(track_uris) = uris {
        body["uris"] = serde_json::json!(track_uris);
    }

    client.put_json(&endpoint, &body).await
}
