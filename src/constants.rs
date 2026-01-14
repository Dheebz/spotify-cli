//! Application-wide constants.
//!
//! Centralizes configuration values that are used across multiple modules.

// =============================================================================
// API Configuration
// =============================================================================

/// Base URL for Spotify Web API endpoints.
pub const SPOTIFY_API_BASE_URL: &str = "https://api.spotify.com/v1";

/// Base URL for Spotify authentication endpoints.
pub const SPOTIFY_AUTH_BASE_URL: &str = "https://accounts.spotify.com";

/// Maximum number of retries for rate-limited API requests.
pub const MAX_API_RETRIES: u32 = 3;

// =============================================================================
// OAuth Configuration
// =============================================================================

/// Default port for the OAuth callback server.
pub const DEFAULT_OAUTH_PORT: u16 = 8888;

/// Path where Spotify redirects after authorization.
pub const OAUTH_CALLBACK_PATH: &str = "/callback";

/// Timeout in seconds for waiting for OAuth callback.
pub const OAUTH_CALLBACK_TIMEOUT_SECS: u64 = 300;

/// Length of the PKCE verifier string.
pub const PKCE_VERIFIER_LENGTH: usize = 128;

// =============================================================================
// Token Management
// =============================================================================

/// Buffer in seconds before token expiry to consider it expired.
/// Tokens are refreshed this many seconds before actual expiry.
pub const TOKEN_EXPIRY_BUFFER_SECS: u64 = 60;

// =============================================================================
// Storage
// =============================================================================

/// Application directory name for config storage.
pub const APP_DIR_NAME: &str = "spotify-cli";

/// Filename for storing OAuth tokens.
pub const TOKEN_FILENAME: &str = "token.json";

/// Filename for storing pins.
pub const PINS_FILENAME: &str = "pins.json";

/// Filename for configuration.
pub const CONFIG_FILENAME: &str = "config.toml";

// =============================================================================
// Search
// =============================================================================

/// Default limit for search results per type.
pub const DEFAULT_SEARCH_LIMIT: u8 = 20;

/// Maximum allowed search limit per type.
pub const MAX_SEARCH_LIMIT: u8 = 50;
