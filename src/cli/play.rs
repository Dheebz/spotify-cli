//! Hidden play command for direct URL/URI playback.
use clap::Args;

use crate::AppContext;
use crate::error::Result;

#[derive(Args, Debug)]
pub struct PlayCommand {
    /// Spotify URL or URI to play (e.g., https://open.spotify.com/playlist/... or spotify:playlist:...)
    pub url: String,
}

/// Supported Spotify resource types for playback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResourceType {
    Track,
    Playlist,
    Album,
    Artist,
}

impl ResourceType {
    fn as_str(&self) -> &'static str {
        match self {
            ResourceType::Track => "track",
            ResourceType::Playlist => "playlist",
            ResourceType::Album => "album",
            ResourceType::Artist => "artist",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "track" => Some(ResourceType::Track),
            "playlist" => Some(ResourceType::Playlist),
            "album" => Some(ResourceType::Album),
            "artist" => Some(ResourceType::Artist),
            _ => None,
        }
    }
}

/// Parsed Spotify resource with type and ID.
struct SpotifyResource {
    resource_type: ResourceType,
    id: String,
}

impl SpotifyResource {
    fn to_uri(&self) -> String {
        format!("spotify:{}:{}", self.resource_type.as_str(), self.id)
    }
}

pub fn handle(command: PlayCommand, ctx: &AppContext) -> Result<()> {
    let resource = parse_spotify_url(&command.url)
        .ok_or_else(|| anyhow::anyhow!("invalid Spotify URL or URI: {}", command.url))?;

    let playback = ctx.spotify()?.playback();
    let uri = resource.to_uri();

    match resource.resource_type {
        ResourceType::Track => {
            playback.play_track(&uri)?;
            ctx.output
                .action("play", &format!("Playing track {}", resource.id))?;
        }
        ResourceType::Playlist => {
            playback.play_context(&uri)?;
            ctx.output
                .action("play", &format!("Playing playlist {}", resource.id))?;
        }
        ResourceType::Album => {
            playback.play_context(&uri)?;
            ctx.output
                .action("play", &format!("Playing album {}", resource.id))?;
        }
        ResourceType::Artist => {
            playback.play_context(&uri)?;
            ctx.output
                .action("play", &format!("Playing artist {}", resource.id))?;
        }
    }

    Ok(())
}

/// Parse a Spotify URL or URI into a resource type and ID.
///
/// Supports:
/// - URIs: `spotify:track:ID`, `spotify:playlist:ID`, `spotify:album:ID`, `spotify:artist:ID`
/// - URIs with user: `spotify:user:USER:playlist:ID`
/// - URLs: `https://open.spotify.com/track/ID`, etc.
fn parse_spotify_url(input: &str) -> Option<SpotifyResource> {
    let cleaned: String = input.split_whitespace().collect();
    let cleaned = cleaned.trim();

    // Handle Spotify URIs (spotify:type:id)
    if cleaned.starts_with("spotify:") {
        return parse_spotify_uri(cleaned);
    }

    // Handle HTTP URLs
    if cleaned.starts_with("http") {
        return parse_http_url(cleaned);
    }

    None
}

fn parse_spotify_uri(uri: &str) -> Option<SpotifyResource> {
    let parts: Vec<&str> = uri.split(':').collect();

    // Standard format: spotify:type:id or spotify:type:id:extra
    if parts.len() >= 3 {
        // Check for user format: spotify:user:username:type:id
        if parts[1] == "user" && parts.len() >= 5 {
            let resource_type = ResourceType::from_str(parts[3])?;
            let id = split_id(parts[4]);
            return Some(SpotifyResource { resource_type, id });
        }

        // Standard format: spotify:type:id
        let resource_type = ResourceType::from_str(parts[1])?;
        let id = split_id(parts[2]);
        return Some(SpotifyResource { resource_type, id });
    }

    None
}

fn parse_http_url(url_str: &str) -> Option<SpotifyResource> {
    let url = url::Url::parse(url_str).ok()?;

    // Only accept Spotify domains
    let host = url.host_str()?;
    if !host.ends_with("spotify.com") {
        return None;
    }

    let segments: Vec<_> = url.path_segments()?.collect();

    if segments.len() >= 2 {
        let resource_type = ResourceType::from_str(segments[0])?;
        let id = segments[1].to_string();
        return Some(SpotifyResource { resource_type, id });
    }

    None
}

/// Split off query params, fragments, or extra URI segments from an ID.
fn split_id(value: &str) -> String {
    value
        .split([':', '?', '#'])
        .next()
        .unwrap_or(value)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_track_uri() {
        let resource = parse_spotify_url("spotify:track:abc123").unwrap();
        assert_eq!(resource.resource_type, ResourceType::Track);
        assert_eq!(resource.id, "abc123");
    }

    #[test]
    fn parse_playlist_uri() {
        let resource = parse_spotify_url("spotify:playlist:xyz789").unwrap();
        assert_eq!(resource.resource_type, ResourceType::Playlist);
        assert_eq!(resource.id, "xyz789");
    }

    #[test]
    fn parse_album_uri() {
        let resource = parse_spotify_url("spotify:album:def456").unwrap();
        assert_eq!(resource.resource_type, ResourceType::Album);
        assert_eq!(resource.id, "def456");
    }

    #[test]
    fn parse_artist_uri() {
        let resource = parse_spotify_url("spotify:artist:ghi012").unwrap();
        assert_eq!(resource.resource_type, ResourceType::Artist);
        assert_eq!(resource.id, "ghi012");
    }

    #[test]
    fn parse_user_playlist_uri() {
        let resource = parse_spotify_url("spotify:user:alice:playlist:abc123").unwrap();
        assert_eq!(resource.resource_type, ResourceType::Playlist);
        assert_eq!(resource.id, "abc123");
    }

    #[test]
    fn parse_playlist_url() {
        let resource =
            parse_spotify_url("https://open.spotify.com/playlist/37i9dQZEVXbsdW9lIOtMPR").unwrap();
        assert_eq!(resource.resource_type, ResourceType::Playlist);
        assert_eq!(resource.id, "37i9dQZEVXbsdW9lIOtMPR");
    }

    #[test]
    fn parse_track_url() {
        let resource =
            parse_spotify_url("https://open.spotify.com/track/4iV5W9uYEdYUVa79Axb7Rh").unwrap();
        assert_eq!(resource.resource_type, ResourceType::Track);
        assert_eq!(resource.id, "4iV5W9uYEdYUVa79Axb7Rh");
    }

    #[test]
    fn parse_url_with_query_params() {
        let resource =
            parse_spotify_url("https://open.spotify.com/playlist/37i9dQZEVXbsdW9lIOtMPR?si=abc123")
                .unwrap();
        assert_eq!(resource.resource_type, ResourceType::Playlist);
        assert_eq!(resource.id, "37i9dQZEVXbsdW9lIOtMPR");
    }

    #[test]
    fn parse_album_url() {
        let resource =
            parse_spotify_url("https://open.spotify.com/album/4aawyAB9vmqN3uQ7FjRGTy").unwrap();
        assert_eq!(resource.resource_type, ResourceType::Album);
        assert_eq!(resource.id, "4aawyAB9vmqN3uQ7FjRGTy");
    }

    #[test]
    fn parse_artist_url() {
        let resource =
            parse_spotify_url("https://open.spotify.com/artist/0OdUWJ0sBjDrqHygGUXeCF").unwrap();
        assert_eq!(resource.resource_type, ResourceType::Artist);
        assert_eq!(resource.id, "0OdUWJ0sBjDrqHygGUXeCF");
    }

    #[test]
    fn to_uri() {
        let resource = SpotifyResource {
            resource_type: ResourceType::Playlist,
            id: "abc123".to_string(),
        };
        assert_eq!(resource.to_uri(), "spotify:playlist:abc123");
    }

    #[test]
    fn invalid_url_returns_none() {
        assert!(parse_spotify_url("not-a-url").is_none());
        assert!(parse_spotify_url("https://example.com/playlist/123").is_none());
    }
}
