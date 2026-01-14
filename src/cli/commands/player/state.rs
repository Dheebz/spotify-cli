//! Player state commands: status, devices

use crate::endpoints::player::{get_available_devices, transfer_playback};
use crate::http::api::SpotifyApi;
use crate::io::output::{ErrorKind, Response};
use crate::types::{Device, DevicesResponse};

use crate::cli::commands::{now_playing, with_client};

/// Find a device ID by name (case-insensitive partial match)
fn find_device_by_name(devices: &[Device], name: &str) -> Option<String> {
    let name_lower = name.to_lowercase();
    devices
        .iter()
        .find(|d| d.name.to_lowercase().contains(&name_lower))
        .and_then(|d| d.id.clone())
}

pub async fn player_status(id_only: Option<&str>) -> Response {
    with_client(|client| async move {
        match now_playing::get_state(&client).await {
            Ok(state) => {
                if let Some(id_type) = id_only {
                    let id = match id_type {
                        "track" => state.item.as_ref().map(|t| t.id.as_str()),
                        "album" => state.item.as_ref()
                            .and_then(|t| t.album.as_ref())
                            .map(|a| a.id.as_str()),
                        "artist" => state.item.as_ref()
                            .and_then(|t| t.artists.as_ref())
                            .and_then(|artists| artists.first())
                            .map(|a| a.id.as_str()),
                        _ => None,
                    };
                    return match id {
                        Some(id) => Response::success(200, id),
                        None => Response::err(404, "Nothing currently playing", ErrorKind::Player),
                    };
                }

                let message = if state.is_playing { "Playing" } else { "Paused" };
                Response::success_with_payload(
                    200,
                    message,
                    serde_json::to_value(&state).expect("PlaybackState serializes to JSON"),
                )
            }
            Err(e) => {
                if id_only.is_some() {
                    return e;
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
        if transfer_playback::transfer_playback(&client, &device).await.is_ok() {
            return Response::success(204, "Playback transferred");
        }

        match get_available_devices::get_available_devices(&client).await {
            Ok(Some(payload)) => {
                let devices: Result<DevicesResponse, _> = serde_json::from_value(payload);
                match devices {
                    Ok(resp) => {
                        if let Some(id) = find_device_by_name(&resp.devices, &device) {
                            return do_transfer(&client, &id).await;
                        }
                        Response::err(404, "Device not found", ErrorKind::NotFound)
                    }
                    Err(_) => Response::err(500, "Failed to parse devices", ErrorKind::Api),
                }
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
