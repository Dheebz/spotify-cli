use std::io::{Read, Write};
use std::net::TcpListener;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, bail};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use rand::RngCore;
use reqwest::blocking::Client as HttpClient;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use url::Url;

use crate::cache::metadata::MetadataStore;
use crate::cache::metadata::{AuthTokenCache, ClientIdentity, Metadata};
use crate::domain::auth::{AuthScopes, AuthStatus};
use crate::domain::settings::Settings;
use crate::error::Result;

const ACCOUNTS_BASE: &str = "https://accounts.spotify.com";
const API_BASE: &str = "https://api.spotify.com/v1";
const REDIRECT_URI_DEFAULT: &str = "http://127.0.0.1:8888/callback";
const SCOPES: &[&str] = &[
    "user-read-playback-state",
    "user-modify-playback-state",
    "user-read-currently-playing",
    "user-read-playback-position",
    "user-top-read",
    "user-read-recently-played",
    "user-library-modify",
    "user-library-read",
    "user-read-private",
    "user-read-email",
    "user-follow-modify",
    "user-follow-read",
    "playlist-read-private",
    "playlist-read-collaborative",
    "playlist-modify-public",
    "playlist-modify-private",
];

/// OAuth token data returned by Spotify.
#[derive(Debug, Clone)]
pub struct AuthToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<u64>,
    pub scopes: Option<Vec<String>>,
}

/// OAuth login and token refresh service.
#[derive(Debug, Clone)]
pub struct AuthService {
    store: MetadataStore,
}

impl AuthService {
    pub fn new(store: MetadataStore) -> Self {
        Self { store }
    }

    pub fn login_oauth(&self, client_id: String) -> Result<()> {
        self.login_oauth_with_redirect(client_id, REDIRECT_URI_DEFAULT)
    }

    pub fn login_oauth_with_redirect(&self, client_id: String, redirect_uri: &str) -> Result<()> {
        let code_verifier = pkce_verifier();
        let code_challenge = pkce_challenge(&code_verifier);
        let state = oauth_state();
        let authorize_url = build_authorize_url(&client_id, redirect_uri, &state, &code_challenge)?;

        println!("Open this URL to authorize: {}", authorize_url);
        println!("Waiting for Spotify authorization...");

        let code = wait_for_code(redirect_uri, &state)?;
        let token = exchange_code(&client_id, redirect_uri, &code, &code_verifier)?;

        let user_name = if should_fetch_profile() {
            fetch_user_name(&token.access_token).ok()
        } else {
            None
        };
        let metadata = Metadata {
            auth: Some(AuthTokenCache {
                access_token: token.access_token,
                refresh_token: token.refresh_token,
                expires_at: token.expires_at,
                granted_scopes: token.scopes,
            }),
            client: Some(ClientIdentity { client_id }),
            settings: Settings {
                user_name,
                ..Settings::default()
            },
        };

        self.store.save(&metadata)?;
        Ok(())
    }

    pub fn login(&self, token: AuthToken) -> Result<()> {
        let mut metadata = self.store.load()?;
        let user_name = if should_fetch_profile() {
            fetch_user_name(&token.access_token).ok()
        } else {
            None
        };
        metadata.auth = Some(AuthTokenCache {
            access_token: token.access_token,
            refresh_token: token.refresh_token,
            expires_at: token.expires_at,
            granted_scopes: token.scopes.clone(),
        });
        if user_name.is_some() {
            metadata.settings.user_name = user_name;
        }
        self.store.save(&metadata)?;
        Ok(())
    }

    pub fn status(&self) -> Result<AuthStatus> {
        let metadata = self.store.load()?;
        let Some(auth) = metadata.auth else {
            return Ok(AuthStatus {
                logged_in: false,
                expires_at: None,
            });
        };

        Ok(AuthStatus {
            logged_in: !auth.access_token.is_empty(),
            expires_at: auth.expires_at,
        })
    }

    pub fn scopes(&self) -> Result<AuthScopes> {
        let metadata = self.store.load()?;
        let required = SCOPES
            .iter()
            .map(|scope| scope.to_string())
            .collect::<Vec<_>>();
        let granted = metadata
            .auth
            .as_ref()
            .and_then(|auth| auth.granted_scopes.clone());
        let missing = if let Some(granted) = granted.as_ref() {
            required
                .iter()
                .filter(|scope| !granted.iter().any(|value| value == *scope))
                .cloned()
                .collect()
        } else {
            Vec::new()
        };

        Ok(AuthScopes {
            required,
            granted,
            missing,
        })
    }

    #[allow(clippy::collapsible_if)]
    pub fn token(&self) -> Result<AuthToken> {
        let metadata = self.store.load()?;
        let Some(mut auth) = metadata.auth else {
            bail!("not logged in; run `spotify auth login`");
        };

        if token_needs_refresh(auth.expires_at) {
            if let (Some(refresh), Some(client)) = (auth.refresh_token.clone(), metadata.client) {
                let refreshed = refresh_token(&client.client_id, &refresh)?;
                auth.access_token = refreshed.access_token;
                auth.expires_at = refreshed.expires_at;
                if refreshed.refresh_token.is_some() {
                    auth.refresh_token = refreshed.refresh_token;
                }
                if refreshed.scopes.is_some() {
                    auth.granted_scopes = refreshed.scopes;
                }
                let updated = Metadata {
                    auth: Some(auth.clone()),
                    client: Some(client),
                    settings: metadata.settings,
                };
                self.store.save(&updated)?;
            }
        }

        if token_needs_refresh(auth.expires_at) {
            bail!("token expired; run `spotify auth login`");
        }

        Ok(AuthToken {
            access_token: auth.access_token,
            refresh_token: auth.refresh_token,
            expires_at: auth.expires_at,
            scopes: auth.granted_scopes,
        })
    }

    pub fn clear(&self) -> Result<()> {
        let metadata = Metadata {
            auth: None,
            client: None,
            settings: Settings::default(),
        };
        self.store.save(&metadata)?;
        Ok(())
    }

    pub fn country(&self) -> Result<Option<String>> {
        let metadata = self.store.load()?;
        Ok(metadata.settings.country)
    }

    pub fn set_country(&self, country: Option<String>) -> Result<()> {
        let mut metadata = self.store.load()?;
        metadata.settings.country = country;
        self.store.save(&metadata)?;
        Ok(())
    }

    pub fn user_name(&self) -> Result<Option<String>> {
        let metadata = self.store.load()?;
        Ok(metadata.settings.user_name)
    }

    pub fn set_user_name(&self, user_name: Option<String>) -> Result<()> {
        let mut metadata = self.store.load()?;
        metadata.settings.user_name = user_name;
        self.store.save(&metadata)?;
        Ok(())
    }

    #[allow(clippy::collapsible_if)]
    pub fn ensure_user_name(&self) -> Result<Option<String>> {
        let mut metadata = self.store.load()?;
        if metadata.settings.user_name.is_some() {
            return Ok(metadata.settings.user_name);
        }

        if !should_fetch_profile() {
            return Ok(None);
        }

        if let Some(auth) = metadata.auth.as_ref() {
            if let Ok(user_name) = fetch_user_name(&auth.access_token) {
                metadata.settings.user_name = Some(user_name.clone());
                self.store.save(&metadata)?;
                return Ok(Some(user_name));
            }
        }

        Ok(None)
    }
}

fn oauth_state() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    let token = URL_SAFE_NO_PAD.encode(bytes);
    format!("spotify-cli-{token}")
}

fn build_authorize_url(
    client_id: &str,
    redirect_uri: &str,
    state: &str,
    code_challenge: &str,
) -> Result<String> {
    let scope = SCOPES.join(" ");
    let encoded_scope = urlencoding::encode(&scope);
    let encoded_redirect = urlencoding::encode(redirect_uri);

    Ok(format!(
        "{ACCOUNTS_BASE}/authorize?response_type=code&client_id={client_id}&scope={encoded_scope}&redirect_uri={encoded_redirect}&state={state}&code_challenge_method=S256&code_challenge={code_challenge}"
    ))
}

fn wait_for_code(redirect_uri: &str, expected_state: &str) -> Result<String> {
    let url = Url::parse(redirect_uri)?;
    if url.scheme() != "http" {
        bail!("redirect URI must use http");
    }

    let host = url.host_str().unwrap_or("127.0.0.1");
    let host = if host == "localhost" {
        "127.0.0.1"
    } else {
        host
    };
    if !matches!(host, "127.0.0.1" | "::1") {
        bail!("redirect URI must use a loopback host");
    }
    let port = url.port_or_known_default().unwrap_or(8888);
    let path = url.path().to_string();

    let listener = TcpListener::bind((host, port))
        .with_context(|| format!("unable to bind redirect listener on {host}:{port}"))?;
    let (mut stream, _) = listener
        .accept()
        .context("failed to accept redirect connection")?;

    let mut buffer = [0u8; 4096];
    let size = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..size]);
    let first_line = request.lines().next().unwrap_or("");
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("");

    if method != "GET" {
        bail!("unexpected redirect method: {method}");
    }

    let (request_path, query) = match target.split_once('?') {
        Some((path, query)) => (path, query),
        None => (target, ""),
    };

    if request_path != path {
        bail!("unexpected redirect path: {request_path}");
    }

    let params = parse_query(query);
    let Some(state) = params.get("state") else {
        bail!("missing state in redirect");
    };

    if state != expected_state {
        bail!("state mismatch during login");
    }

    let Some(code) = params.get("code") else {
        bail!("missing code in redirect");
    };

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nYou can close this window.";
    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(code.to_string())
}

fn parse_query(query: &str) -> std::collections::HashMap<String, String> {
    let mut params = std::collections::HashMap::new();
    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }
        let (key, value) = match pair.split_once('=') {
            Some((key, value)) => (key, value),
            None => (pair, ""),
        };
        let key = urlencoding::decode(key).unwrap_or_else(|_| key.into());
        let value = urlencoding::decode(value).unwrap_or_else(|_| value.into());
        params.insert(key.to_string(), value.to_string());
    }
    params
}

fn exchange_code(
    client_id: &str,
    redirect_uri: &str,
    code: &str,
    code_verifier: &str,
) -> Result<AuthToken> {
    let client = HttpClient::builder().build()?;
    let url = format!("{ACCOUNTS_BASE}/api/token");

    let response = client
        .post(url)
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", redirect_uri),
            ("client_id", client_id),
            ("code_verifier", code_verifier),
        ])
        .send()
        .context("spotify token exchange failed")?;

    if !response.status().is_success() {
        bail!("spotify token exchange failed: {}", response.status());
    }

    let payload: TokenResponse = response.json()?;
    Ok(AuthToken {
        access_token: payload.access_token,
        refresh_token: payload.refresh_token,
        expires_at: Some(unix_time() + payload.expires_in),
        scopes: payload.scope.map(parse_scopes),
    })
}

fn refresh_token(client_id: &str, refresh_token: &str) -> Result<AuthToken> {
    let client = HttpClient::builder().build()?;
    let url = format!("{ACCOUNTS_BASE}/api/token");

    let response = client
        .post(url)
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", client_id),
        ])
        .send()
        .context("spotify token refresh failed")?;

    if !response.status().is_success() {
        bail!("spotify token refresh failed: {}", response.status());
    }

    let payload: TokenResponse = response.json()?;
    Ok(AuthToken {
        access_token: payload.access_token,
        refresh_token: payload.refresh_token,
        expires_at: Some(unix_time() + payload.expires_in),
        scopes: payload.scope.map(parse_scopes),
    })
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
    #[serde(default)]
    refresh_token: Option<String>,
    #[serde(default)]
    scope: Option<String>,
}

fn unix_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_secs()
}

fn token_needs_refresh(expires_at: Option<u64>) -> bool {
    let Some(expires_at) = expires_at else {
        return false;
    };
    unix_time().saturating_add(60) >= expires_at
}

fn should_fetch_profile() -> bool {
    std::env::var("SPOTIFY_CLI_SKIP_PROFILE").is_err()
}

fn fetch_user_name(access_token: &str) -> Result<String> {
    let client = HttpClient::builder().build()?;
    let url = format!("{API_BASE}/me");
    let response = client.get(url).bearer_auth(access_token).send()?;
    if !response.status().is_success() {
        bail!("spotify profile request failed: {}", response.status());
    }
    let payload: UserProfile = response.json()?;
    Ok(payload
        .display_name
        .or(payload.id)
        .unwrap_or_else(|| "You".to_string()))
}

fn parse_scopes(scope: String) -> Vec<String> {
    scope
        .split_whitespace()
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string())
        .collect()
}

#[derive(Deserialize)]
struct UserProfile {
    display_name: Option<String>,
    id: Option<String>,
}

fn pkce_verifier() -> String {
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

fn pkce_challenge(verifier: &str) -> String {
    let digest = Sha256::digest(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(digest)
}
