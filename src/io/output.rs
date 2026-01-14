//! Response types and output functions

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::registry::format_payload_with_kind as format_payload;
use crate::http::client::HttpError;

/// Payload type hint for reliable formatter matching.
///
/// This eliminates brittle payload inspection by explicitly declaring the payload type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PayloadKind {
    // Player-related
    PlayerStatus,
    Queue,
    Devices,
    PlayHistory,

    // Search
    SearchResults,
    CombinedSearch,
    Pins,

    // Resources
    Track,
    Album,
    Artist,
    Playlist,
    Show,
    Episode,
    Audiobook,
    Chapter,
    Category,
    User,

    // Lists
    TrackList,
    AlbumList,
    ArtistList,
    PlaylistList,
    ShowList,
    EpisodeList,
    AudiobookList,
    ChapterList,
    CategoryList,
    TopTracks,
    TopArtists,
    ArtistTopTracks,
    RelatedArtists,
    NewReleases,
    FeaturedPlaylists,
    FollowedArtists,

    // Library
    SavedTracks,
    SavedAlbums,
    SavedShows,
    SavedEpisodes,
    SavedAudiobooks,
    LibraryCheck,

    // Other
    Markets,
    Generic,
}

/// Type-safe error categories
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorKind {
    /// Network/connectivity issues
    Network,
    /// Spotify API returned an error
    Api,
    /// Authentication required or failed
    Auth,
    /// Resource not found
    NotFound,
    /// Permission denied
    Forbidden,
    /// Rate limited by Spotify
    RateLimited,
    /// Invalid input from user
    Validation,
    /// Local storage error
    Storage,
    /// Configuration error
    Config,
    /// Player-specific error
    Player,
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorKind::Network => "network_error",
            ErrorKind::Api => "api_error",
            ErrorKind::Auth => "auth_error",
            ErrorKind::NotFound => "not_found",
            ErrorKind::Forbidden => "forbidden",
            ErrorKind::RateLimited => "rate_limited",
            ErrorKind::Validation => "validation_error",
            ErrorKind::Storage => "storage_error",
            ErrorKind::Config => "config_error",
            ErrorKind::Player => "player_error",
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Success,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "Response should be returned or printed, not ignored"]
pub struct Response {
    pub status: Status,
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_kind: Option<PayloadKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl Response {
    pub fn success(code: u16, message: impl Into<String>) -> Self {
        Self {
            status: Status::Success,
            code,
            message: message.into(),
            payload: None,
            payload_kind: None,
            error: None,
        }
    }

    pub fn success_with_payload(code: u16, message: impl Into<String>, payload: Value) -> Self {
        Self {
            status: Status::Success,
            code,
            message: message.into(),
            payload: Some(payload),
            payload_kind: None,
            error: None,
        }
    }

    /// Create a success response with typed payload for reliable formatter matching.
    ///
    /// Use this instead of `success_with_payload` when you know the exact payload type.
    pub fn success_typed(
        code: u16,
        message: impl Into<String>,
        kind: PayloadKind,
        payload: Value,
    ) -> Self {
        Self {
            status: Status::Success,
            code,
            message: message.into(),
            payload: Some(payload),
            payload_kind: Some(kind),
            error: None,
        }
    }

    /// Create an error response with ErrorKind
    pub fn err(code: u16, message: impl Into<String>, kind: ErrorKind) -> Self {
        Self {
            status: Status::Error,
            code,
            message: message.into(),
            payload: None,
            payload_kind: None,
            error: Some(ErrorDetail {
                kind: kind.to_string(),
                details: None,
            }),
        }
    }

    /// Create an error response with ErrorKind and details
    pub fn err_with_details(
        code: u16,
        message: impl Into<String>,
        kind: ErrorKind,
        details: impl Into<String>,
    ) -> Self {
        Self {
            status: Status::Error,
            code,
            message: message.into(),
            payload: None,
            payload_kind: None,
            error: Some(ErrorDetail {
                kind: kind.to_string(),
                details: Some(details.into()),
            }),
        }
    }

    /// Create a Response from an HttpError, preserving the original status code
    pub fn from_http_error(err: &HttpError, context: &str) -> Self {
        let kind = match err {
            HttpError::Network(_) => ErrorKind::Network,
            HttpError::Unauthorized => ErrorKind::Auth,
            HttpError::Forbidden => ErrorKind::Forbidden,
            HttpError::NotFound => ErrorKind::NotFound,
            HttpError::RateLimited { .. } => ErrorKind::RateLimited,
            HttpError::Api { .. } => ErrorKind::Api,
        };

        let status_text = match err.status_code() {
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            429 => "Rate Limited",
            500 => "Internal Server Error",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            _ => "",
        };

        let message = if status_text.is_empty() {
            format!("{} ({})", context, err.status_code())
        } else {
            format!("{}: {} {}", context, err.status_code(), status_text)
        };

        Self {
            status: Status::Error,
            code: err.status_code(),
            message,
            payload: None,
            payload_kind: None,
            error: Some(ErrorDetail {
                kind: kind.to_string(),
                details: Some(err.user_message().to_string()),
            }),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| {
            r#"{"status":"error","code":500,"message":"Failed to serialize response"}"#.to_string()
        })
    }
}

/// Create an API error response from HttpError (preserves status code)
#[macro_export]
macro_rules! api_error {
    ($ctx:expr, $err:expr) => {
        $crate::io::output::Response::from_http_error(&$err, $ctx)
    };
}

/// Create a storage error response (500, ErrorKind::Storage)
#[macro_export]
macro_rules! storage_error {
    ($msg:expr, $err:expr) => {
        $crate::io::output::Response::err_with_details(
            500,
            $msg,
            $crate::io::output::ErrorKind::Storage,
            $err.to_string(),
        )
    };
}

/// Create an auth error response (401, ErrorKind::Auth)
#[macro_export]
macro_rules! auth_error {
    ($msg:expr, $err:expr) => {
        $crate::io::output::Response::err_with_details(
            401,
            $msg,
            $crate::io::output::ErrorKind::Auth,
            $err.to_string(),
        )
    };
}

pub fn print_json(response: &Response) {
    println!("{}", response.to_json());
}

pub fn print_human(response: &Response) {
    match &response.status {
        Status::Error => {
            eprintln!("Error: {}", response.message);
            if let Some(err) = &response.error
                && let Some(details) = &err.details {
                    eprintln!("  {}", details);
                }
        }
        Status::Success => {
            if let Some(payload) = &response.payload {
                format_payload(payload, &response.message, response.payload_kind);
            } else {
                println!("{}", response.message);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_response_serializes() {
        let resp = Response::success(200, "OK");
        let json = resp.to_json();
        assert!(json.contains(r#""status":"success""#));
        assert!(json.contains(r#""code":200"#));
    }

    #[test]
    fn error_response_includes_error_detail() {
        let resp = Response::err(401, "Unauthorized", ErrorKind::Auth);
        let json = resp.to_json();
        assert!(json.contains(r#""status":"error""#));
        assert!(json.contains(r#""kind":"auth_error""#));
    }

    #[test]
    fn payload_skipped_when_none() {
        let resp = Response::success(200, "OK");
        let json = resp.to_json();
        assert!(!json.contains("payload"));
    }

    #[test]
    fn payload_included_when_present() {
        let payload = serde_json::json!({"track": "test"});
        let resp = Response::success_with_payload(200, "OK", payload);
        let json = resp.to_json();
        assert!(json.contains("payload"));
        assert!(json.contains("track"));
    }

    #[test]
    fn error_kind_serializes_to_snake_case() {
        assert_eq!(ErrorKind::NotFound.as_str(), "not_found");
        assert_eq!(ErrorKind::RateLimited.as_str(), "rate_limited");
        assert_eq!(ErrorKind::Api.as_str(), "api_error");
    }

    #[test]
    fn success_typed_includes_payload_kind() {
        let payload = serde_json::json!({"name": "Test Track"});
        let resp = Response::success_typed(200, "Track info", PayloadKind::Track, payload);
        assert!(resp.payload_kind.is_some());
        assert_eq!(resp.payload_kind.unwrap(), PayloadKind::Track);
    }

    #[test]
    fn success_typed_serializes_payload_kind() {
        let payload = serde_json::json!({"name": "Test"});
        let resp = Response::success_typed(200, "OK", PayloadKind::Playlist, payload);
        let json = resp.to_json();
        assert!(json.contains("payload_kind"));
        assert!(json.contains("playlist"));
    }

    #[test]
    fn success_without_typed_has_no_payload_kind() {
        let payload = serde_json::json!({"name": "Test"});
        let resp = Response::success_with_payload(200, "OK", payload);
        assert!(resp.payload_kind.is_none());
        let json = resp.to_json();
        assert!(!json.contains("payload_kind"));
    }

    #[test]
    fn payload_kind_variants() {
        // Test that PayloadKind serializes correctly
        let kinds = vec![
            (PayloadKind::PlayerStatus, "player_status"),
            (PayloadKind::Queue, "queue"),
            (PayloadKind::Track, "track"),
            (PayloadKind::Album, "album"),
            (PayloadKind::Artist, "artist"),
            (PayloadKind::Playlist, "playlist"),
            (PayloadKind::SavedTracks, "saved_tracks"),
            (PayloadKind::LibraryCheck, "library_check"),
        ];
        for (kind, expected) in kinds {
            let serialized = serde_json::to_string(&kind).unwrap();
            assert!(serialized.contains(expected), "Expected {} in {}", expected, serialized);
        }
    }

    #[test]
    fn error_response_has_no_payload_kind() {
        let resp = Response::err(404, "Not found", ErrorKind::NotFound);
        assert!(resp.payload_kind.is_none());
    }

    #[test]
    fn err_with_details_includes_details() {
        let resp = Response::err_with_details(
            500,
            "Storage failed",
            ErrorKind::Storage,
            "Disk full",
        );
        assert!(resp.error.is_some());
        let error = resp.error.unwrap();
        assert_eq!(error.kind, "storage_error");
        assert_eq!(error.details, Some("Disk full".to_string()));
    }

    #[test]
    fn all_error_kinds_as_str() {
        assert_eq!(ErrorKind::Network.as_str(), "network_error");
        assert_eq!(ErrorKind::Api.as_str(), "api_error");
        assert_eq!(ErrorKind::Auth.as_str(), "auth_error");
        assert_eq!(ErrorKind::NotFound.as_str(), "not_found");
        assert_eq!(ErrorKind::Forbidden.as_str(), "forbidden");
        assert_eq!(ErrorKind::RateLimited.as_str(), "rate_limited");
        assert_eq!(ErrorKind::Validation.as_str(), "validation_error");
        assert_eq!(ErrorKind::Storage.as_str(), "storage_error");
        assert_eq!(ErrorKind::Config.as_str(), "config_error");
        assert_eq!(ErrorKind::Player.as_str(), "player_error");
    }

    #[test]
    fn error_kind_display() {
        assert_eq!(format!("{}", ErrorKind::Network), "network_error");
        assert_eq!(format!("{}", ErrorKind::Auth), "auth_error");
    }

    #[test]
    fn from_http_error_unauthorized() {
        let http_err = HttpError::Unauthorized;
        let resp = Response::from_http_error(&http_err, "Auth check");
        assert_eq!(resp.code, 401);
        assert!(resp.message.contains("Unauthorized"));
        assert!(resp.error.is_some());
    }

    #[test]
    fn from_http_error_not_found() {
        let http_err = HttpError::NotFound;
        let resp = Response::from_http_error(&http_err, "Get resource");
        assert_eq!(resp.code, 404);
        assert!(resp.message.contains("Not Found"));
    }

    #[test]
    fn from_http_error_rate_limited() {
        let http_err = HttpError::RateLimited { retry_after_secs: 30 };
        let resp = Response::from_http_error(&http_err, "API call");
        assert_eq!(resp.code, 429);
        assert!(resp.message.contains("Rate Limited"));
    }

    #[test]
    fn from_http_error_forbidden() {
        let http_err = HttpError::Forbidden;
        let resp = Response::from_http_error(&http_err, "Action");
        assert_eq!(resp.code, 403);
        assert!(resp.message.contains("Forbidden"));
    }

    #[test]
    fn from_http_error_api_error() {
        let http_err = HttpError::Api { status: 500, message: "Server error".to_string() };
        let resp = Response::from_http_error(&http_err, "Request");
        assert_eq!(resp.code, 500);
        assert!(resp.message.contains("Internal Server Error"));
    }

    #[test]
    fn status_serialization() {
        let success = serde_json::to_string(&Status::Success).unwrap();
        assert!(success.contains("success"));

        let error = serde_json::to_string(&Status::Error).unwrap();
        assert!(error.contains("error"));
    }

    #[test]
    fn more_payload_kind_variants() {
        let kinds = vec![
            (PayloadKind::Devices, "devices"),
            (PayloadKind::PlayHistory, "play_history"),
            (PayloadKind::SearchResults, "search_results"),
            (PayloadKind::Show, "show"),
            (PayloadKind::Episode, "episode"),
            (PayloadKind::Audiobook, "audiobook"),
            (PayloadKind::Chapter, "chapter"),
            (PayloadKind::Category, "category"),
            (PayloadKind::User, "user"),
            (PayloadKind::TrackList, "track_list"),
            (PayloadKind::AlbumList, "album_list"),
            (PayloadKind::Markets, "markets"),
            (PayloadKind::Generic, "generic"),
        ];
        for (kind, expected) in kinds {
            let serialized = serde_json::to_string(&kind).unwrap();
            assert!(serialized.contains(expected), "Expected {} in {}", expected, serialized);
        }
    }

    #[test]
    fn print_json_outputs_valid_json() {
        let resp = Response::success(200, "Test");
        print_json(&resp);
    }

    #[test]
    fn print_human_success_with_payload() {
        let payload = serde_json::json!({"name": "Test"});
        let resp = Response::success_with_payload(200, "Message", payload);
        print_human(&resp);
    }

    #[test]
    fn print_human_success_without_payload() {
        let resp = Response::success(200, "Simple message");
        print_human(&resp);
    }

    #[test]
    fn print_human_error_with_details() {
        let resp = Response::err_with_details(
            500,
            "Operation failed",
            ErrorKind::Api,
            "Detailed error info",
        );
        print_human(&resp);
    }

    #[test]
    fn print_human_error_without_details() {
        let resp = Response::err(404, "Not found", ErrorKind::NotFound);
        print_human(&resp);
    }

    #[test]
    fn from_http_error_api_includes_details() {
        let http_err = HttpError::Api { status: 500, message: "Server error".to_string() };
        let resp = Response::from_http_error(&http_err, "Request failed");
        assert!(resp.error.is_some());
        let error = resp.error.unwrap();
        assert_eq!(error.kind, "api_error");
        assert!(error.details.is_some());
    }

    #[test]
    fn from_http_error_unusual_status() {
        let http_err = HttpError::Api { status: 418, message: "I'm a teapot".to_string() };
        let resp = Response::from_http_error(&http_err, "Request");
        assert_eq!(resp.code, 418);
        // Non-standard status codes should use fallback format
        assert!(resp.message.contains("418"));
    }

    #[test]
    fn from_http_error_bad_gateway() {
        let http_err = HttpError::Api { status: 502, message: "Bad Gateway".to_string() };
        let resp = Response::from_http_error(&http_err, "Request");
        assert_eq!(resp.code, 502);
        assert!(resp.message.contains("Bad Gateway"));
    }

    #[test]
    fn from_http_error_service_unavailable() {
        let http_err = HttpError::Api { status: 503, message: "Unavailable".to_string() };
        let resp = Response::from_http_error(&http_err, "Request");
        assert_eq!(resp.code, 503);
        assert!(resp.message.contains("Service Unavailable"));
    }

    #[test]
    fn from_http_error_bad_request() {
        let http_err = HttpError::Api { status: 400, message: "Invalid params".to_string() };
        let resp = Response::from_http_error(&http_err, "Validation");
        assert_eq!(resp.code, 400);
        assert!(resp.message.contains("Bad Request"));
    }

    #[test]
    fn print_human_with_typed_payload() {
        let payload = serde_json::json!({"name": "Test Track"});
        let resp = Response::success_typed(200, "Track", PayloadKind::Track, payload);
        print_human(&resp);
    }
}
