use super::{init_token_store, load_config};
use crate::io::output::{ErrorKind, Response};
use crate::oauth::flow::OAuthFlow;
use crate::{auth_error, storage_error};

pub async fn auth_login(force: bool) -> Response {
    let config = match load_config() {
        Ok(c) => c,
        Err(e) => return e,
    };

    let token_store = match init_token_store() {
        Ok(s) => s,
        Err(e) => return e,
    };

    // If not forcing, check if we already have a valid token or can refresh
    if !force
        && let Ok(token) = token_store.load() {
            if !token.is_expired() {
                // Already have a valid token
                return Response::success_with_payload(
                    200,
                    "Already logged in",
                    serde_json::json!({
                        "expires_in": token.seconds_until_expiry()
                    }),
                );
            }

            // Token expired, try to refresh
            if let Some(refresh_token) = &token.refresh_token {
                let flow = OAuthFlow::new(config.client_id().to_string());
                match flow.refresh(refresh_token).await {
                    Ok(new_token) => {
                        if let Err(e) = token_store.save(&new_token) {
                            return storage_error!("Failed to save token", e);
                        }
                        return Response::success_with_payload(
                            200,
                            "Token refreshed",
                            serde_json::json!({
                                "expires_in": new_token.seconds_until_expiry()
                            }),
                        );
                    }
                    Err(e) => {
                        // Refresh failed - log and fall through to browser flow
                        eprintln!("Note: Token refresh failed ({}), opening browser login...", e);
                    }
                }
            }
        }

    // No valid token or force flag - do browser flow
    let flow = OAuthFlow::new(config.client_id().to_string());
    match flow.authenticate().await {
        Ok(token) => {
            if let Err(e) = token_store.save(&token) {
                return storage_error!("Failed to save token", e);
            }
            Response::success_with_payload(
                200,
                "Login successful",
                serde_json::json!({
                    "expires_in": token.seconds_until_expiry()
                }),
            )
        }
        Err(e) => auth_error!("Login failed", e),
    }
}

pub async fn auth_logout() -> Response {
    let token_store = match init_token_store() {
        Ok(s) => s,
        Err(e) => return e,
    };

    if !token_store.exists() {
        return Response::success(200, "Already logged out");
    }

    match token_store.delete() {
        Ok(_) => Response::success(200, "Logged out successfully"),
        Err(e) => storage_error!("Failed to delete token", e),
    }
}

pub async fn auth_refresh() -> Response {
    let config = match load_config() {
        Ok(c) => c,
        Err(e) => return e,
    };

    let token_store = match init_token_store() {
        Ok(s) => s,
        Err(e) => return e,
    };

    let token = match token_store.load() {
        Ok(t) => t,
        Err(_) => return Response::err(401, "Not logged in. Run: spotify-cli auth login", ErrorKind::Auth),
    };

    let refresh_token = match &token.refresh_token {
        Some(t) => t,
        None => return Response::err(401, "No refresh token available", ErrorKind::Auth),
    };

    let flow = OAuthFlow::new(config.client_id().to_string());
    match flow.refresh(refresh_token).await {
        Ok(new_token) => {
            if let Err(e) = token_store.save(&new_token) {
                return storage_error!("Failed to save token", e);
            }
            Response::success_with_payload(
                200,
                "Token refreshed",
                serde_json::json!({
                    "expires_in": new_token.seconds_until_expiry()
                }),
            )
        }
        Err(e) => auth_error!("Failed to refresh token", e),
    }
}

pub async fn auth_status() -> Response {
    let token_store = match init_token_store() {
        Ok(s) => s,
        Err(e) => return e,
    };

    if !token_store.exists() {
        return Response::success_with_payload(
            200,
            "Not authenticated",
            serde_json::json!({
                "authenticated": false
            }),
        );
    }

    match token_store.load() {
        Ok(token) => {
            let expired = token.is_expired();
            let expires_in = token.seconds_until_expiry();

            Response::success_with_payload(
                200,
                if expired {
                    "Token expired"
                } else {
                    "Authenticated"
                },
                serde_json::json!({
                    "authenticated": !expired,
                    "expired": expired,
                    "expires_in": expires_in
                }),
            )
        }
        Err(e) => storage_error!("Failed to load token", e),
    }
}
