/// Authentication status returned by `spotify-cli auth status`.
#[derive(Debug, Clone, Copy)]
pub struct AuthStatus {
    pub logged_in: bool,
    pub expires_at: Option<u64>,
}

/// Scope inspection payload for `spotify-cli auth scopes`.
#[derive(Debug, Clone)]
pub struct AuthScopes {
    /// Full set of scopes requested by the CLI.
    pub required: Vec<String>,
    /// Scopes granted by Spotify for the current token.
    pub granted: Option<Vec<String>>,
    /// Required scopes missing from the granted set.
    pub missing: Vec<String>,
}
