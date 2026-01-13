use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Change playlist details (name, description, public status)
pub async fn change_playlist_details(
    client: &SpotifyApi,
    playlist_id: &str,
    name: Option<&str>,
    description: Option<&str>,
    public: Option<bool>,
) -> Result<Option<Value>, HttpError> {
    let mut body = serde_json::json!({});

    if let Some(n) = name {
        body["name"] = serde_json::Value::String(n.to_string());
    }
    if let Some(d) = description {
        body["description"] = serde_json::Value::String(d.to_string());
    }
    if let Some(p) = public {
        body["public"] = serde_json::Value::Bool(p);
    }

    client.put_json(&Endpoint::Playlist { id: playlist_id }.path(), &body).await
}
