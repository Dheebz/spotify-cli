//! Player state commands: status, devices

use crate::endpoints::player::{get_available_devices, get_playback_state, transfer_playback};
use crate::http::api::SpotifyApi;
use crate::io::output::{ErrorKind, Response};
use serde_json::Value;

use crate::cli::commands::with_client;

/// Find a device ID by name (case-insensitive partial match)
fn find_device_id(devices: &[Value], name: &str) -> Option<String> {
    let name_lower = name.to_lowercase();
    for dev in devices {
        if let Some(dev_name) = dev.get("name").and_then(|n| n.as_str())
            && dev_name.to_lowercase().contains(&name_lower) {
                return dev.get("id").and_then(|i| i.as_str()).map(|s| s.to_string());
            }
    }
    None
}

pub async fn player_status(id_only: Option<&str>) -> Response {
    with_client(|client| async move {
        match get_playback_state::get_playback_state(&client).await {
            Ok(Some(payload)) => {
                // If --id-only is specified, just output the ID
                if let Some(id_type) = id_only {
                    let id = match id_type {
                        "track" => payload
                            .get("item")
                            .and_then(|i| i.get("id"))
                            .and_then(|v| v.as_str()),
                        "album" => payload
                            .get("item")
                            .and_then(|i| i.get("album"))
                            .and_then(|a| a.get("id"))
                            .and_then(|v| v.as_str()),
                        "artist" => payload
                            .get("item")
                            .and_then(|i| i.get("artists"))
                            .and_then(|a| a.as_array())
                            .and_then(|arr| arr.first())
                            .and_then(|a| a.get("id"))
                            .and_then(|v| v.as_str()),
                        _ => None,
                    };
                    return match id {
                        Some(id) => Response::success(200, id),
                        None => Response::err(404, "Nothing currently playing", ErrorKind::Player),
                    };
                }

                let is_playing = payload
                    .get("is_playing")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                Response::success_with_payload(
                    200,
                    if is_playing { "Playing" } else { "Paused" },
                    payload,
                )
            }
            Ok(None) => {
                if id_only.is_some() {
                    return Response::err(404, "Nothing currently playing", ErrorKind::Player);
                }
                Response::success_with_payload(
                    204,
                    "No active playback",
                    serde_json::json!({
                        "is_playing": false,
                        "active": false
                    }),
                )
            }
            Err(e) => Response::from_http_error(&e, "Failed to get playback status"),
        }
    })
    .await
}

pub async fn player_devices_list() -> Response {
    with_client(|client| async move {
        match get_available_devices::get_available_devices(&client).await {
            Ok(Some(payload)) => Response::success_with_payload(200, "Available devices", payload),
            Ok(None) => Response::success_with_payload(
                200,
                "No devices available",
                serde_json::json!({ "devices": [] }),
            ),
            Err(e) => Response::from_http_error(&e, "Failed to get devices"),
        }
    })
    .await
}

pub async fn player_devices_transfer(device: &str) -> Response {
    let device = device.to_string();
    with_client(|client| async move {
        // First, try to transfer by device ID directly
        if transfer_playback::transfer_playback(&client, &device).await.is_ok() {
            return Response::success(204, "Playback transferred");
        }
        // Fall through to try by name

        // Try to find device by name
        match get_available_devices::get_available_devices(&client).await {
            Ok(Some(payload)) => {
                if let Some(devices) = payload.get("devices").and_then(|d| d.as_array())
                    && let Some(id) = find_device_id(devices, &device) {
                        return do_transfer(&client, &id).await;
                    }
                Response::err(404, "Device not found", ErrorKind::NotFound)
            }
            Ok(None) => Response::err(404, "No devices available", ErrorKind::Player),
            Err(e) => Response::from_http_error(&e, "Failed to get devices"),
        }
    })
    .await
}

async fn do_transfer(client: &SpotifyApi, device_id: &str) -> Response {
    match transfer_playback::transfer_playback(client, device_id).await {
        Ok(_) => Response::success(204, "Playback transferred"),
        Err(e) => Response::from_http_error(&e, "Failed to transfer playback"),
    }
}
