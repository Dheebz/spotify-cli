//! Resource command macros to reduce boilerplate
//!
//! These macros generate common patterns for CRUD operations on Spotify resources.

/// Generate a "get single resource by ID" command
///
/// # Example
/// ```ignore
/// resource_get!(show_get, get_show::get_show, "Show");
/// ```
#[macro_export]
macro_rules! resource_get {
    ($fn_name:ident, $endpoint_mod:ident :: $endpoint_fn:ident, $resource_name:literal) => {
        pub async fn $fn_name(id: &str) -> $crate::io::output::Response {
            let id = $crate::cli::commands::extract_id(id);

            $crate::cli::commands::with_client(|client| async move {
                match $endpoint_mod::$endpoint_fn(&client, &id).await {
                    Ok(Some(payload)) => $crate::io::output::Response::success_with_payload(
                        200,
                        concat!($resource_name, " details"),
                        payload,
                    ),
                    Ok(None) => $crate::io::output::Response::err(
                        404,
                        concat!($resource_name, " not found"),
                        $crate::io::output::ErrorKind::NotFound,
                    ),
                    Err(e) => $crate::io::output::Response::from_http_error(
                        &e,
                        concat!("Failed to get ", $resource_name),
                    ),
                }
            })
            .await
        }
    };
}

/// Generate a "list resources with pagination" command
///
/// # Example
/// ```ignore
/// resource_list!(show_list, get_users_saved_shows::get_users_saved_shows, "Saved shows");
/// ```
#[macro_export]
macro_rules! resource_list {
    ($fn_name:ident, $endpoint_mod:ident :: $endpoint_fn:ident, $success_msg:literal) => {
        pub async fn $fn_name(limit: u8, offset: u32) -> $crate::io::output::Response {
            $crate::cli::commands::with_client(|client| async move {
                match $endpoint_mod::$endpoint_fn(&client, Some(limit), Some(offset)).await {
                    Ok(Some(payload)) => {
                        $crate::io::output::Response::success_with_payload(200, $success_msg, payload)
                    }
                    Ok(None) => $crate::io::output::Response::success_with_payload(
                        200,
                        $success_msg,
                        serde_json::json!({ "items": [] }),
                    ),
                    Err(e) => $crate::io::output::Response::from_http_error(
                        &e,
                        concat!("Failed to get ", $success_msg),
                    ),
                }
            })
            .await
        }
    };
}

/// Generate a "list sub-resources with ID and pagination" command (e.g., show episodes, audiobook chapters)
///
/// # Example
/// ```ignore
/// resource_list_with_id!(show_episodes, get_show_episodes::get_show_episodes, "Show episodes");
/// ```
#[macro_export]
macro_rules! resource_list_with_id {
    ($fn_name:ident, $endpoint_mod:ident :: $endpoint_fn:ident, $success_msg:literal, $empty_msg:literal) => {
        pub async fn $fn_name(id: &str, limit: u8, offset: u32) -> $crate::io::output::Response {
            let id = $crate::cli::commands::extract_id(id);

            $crate::cli::commands::with_client(|client| async move {
                match $endpoint_mod::$endpoint_fn(&client, &id, Some(limit), Some(offset)).await {
                    Ok(Some(payload)) => {
                        $crate::io::output::Response::success_with_payload(200, $success_msg, payload)
                    }
                    Ok(None) => $crate::io::output::Response::success_with_payload(
                        200,
                        $empty_msg,
                        serde_json::json!({ "items": [] }),
                    ),
                    Err(e) => $crate::io::output::Response::from_http_error(
                        &e,
                        concat!("Failed to get ", $success_msg),
                    ),
                }
            })
            .await
        }
    };
}

/// Generate a "save resources" command
///
/// # Example
/// ```ignore
/// resource_save!(show_save, save_shows_for_current_user::save_shows, "show");
/// ```
#[macro_export]
macro_rules! resource_save {
    ($fn_name:ident, $endpoint_mod:ident :: $endpoint_fn:ident, $resource_name:literal) => {
        pub async fn $fn_name(ids: &[String]) -> $crate::io::output::Response {
            let ids = ids.to_vec();
            let count = ids.len();

            $crate::cli::commands::with_client(|client| async move {
                match $endpoint_mod::$endpoint_fn(&client, &ids).await {
                    Ok(_) => $crate::io::output::Response::success(
                        200,
                        &format!(concat!("Saved {} ", $resource_name, "(s)"), count),
                    ),
                    Err(e) => $crate::io::output::Response::from_http_error(
                        &e,
                        concat!("Failed to save ", $resource_name, "s"),
                    ),
                }
            })
            .await
        }
    };
}

/// Generate a "remove resources" command
///
/// # Example
/// ```ignore
/// resource_remove!(show_remove, remove_users_saved_shows::remove_shows, "show");
/// ```
#[macro_export]
macro_rules! resource_remove {
    ($fn_name:ident, $endpoint_mod:ident :: $endpoint_fn:ident, $resource_name:literal) => {
        pub async fn $fn_name(ids: &[String]) -> $crate::io::output::Response {
            let ids = ids.to_vec();
            let count = ids.len();

            $crate::cli::commands::with_client(|client| async move {
                match $endpoint_mod::$endpoint_fn(&client, &ids).await {
                    Ok(_) => $crate::io::output::Response::success(
                        200,
                        &format!(concat!("Removed {} ", $resource_name, "(s)"), count),
                    ),
                    Err(e) => $crate::io::output::Response::from_http_error(
                        &e,
                        concat!("Failed to remove ", $resource_name, "s"),
                    ),
                }
            })
            .await
        }
    };
}

/// Generate a "check if resources are saved" command
///
/// # Example
/// ```ignore
/// resource_check!(show_check, check_users_saved_shows::check_saved_shows);
/// ```
#[macro_export]
macro_rules! resource_check {
    ($fn_name:ident, $endpoint_mod:ident :: $endpoint_fn:ident) => {
        pub async fn $fn_name(ids: &[String]) -> $crate::io::output::Response {
            let ids = ids.to_vec();

            $crate::cli::commands::with_client(|client| async move {
                match $endpoint_mod::$endpoint_fn(&client, &ids).await {
                    Ok(Some(payload)) => {
                        $crate::io::output::Response::success_with_payload(200, "Check results", payload)
                    }
                    Ok(None) => $crate::io::output::Response::success_with_payload(
                        200,
                        "Check results",
                        serde_json::json!([]),
                    ),
                    Err(e) => {
                        $crate::io::output::Response::from_http_error(&e, "Failed to check")
                    }
                }
            })
            .await
        }
    };
}

