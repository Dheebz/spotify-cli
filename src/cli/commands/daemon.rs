//! Daemon management commands

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::io::output::{ErrorKind, Response};
use crate::rpc::events::EventPoller;
use crate::rpc::{Server, ServerConfig};
use crate::storage::paths;

use tokio::signal;
use tracing::info;

/// Get the PID file path
fn pid_file_path() -> PathBuf {
    paths::pid_file().unwrap_or_else(|_| PathBuf::from("/tmp/spotify-cli.pid"))
}

/// Start the daemon in the background
pub async fn daemon_start() -> Response {
    let pid_file = pid_file_path();

    // Check if already running
    if let Some(pid) = read_pid(&pid_file) {
        if is_process_running(pid) {
            return Response::err(409, &format!("Daemon already running (PID {})", pid), ErrorKind::Validation);
        }
        // Stale PID file, remove it
        let _ = fs::remove_file(&pid_file);
    }

    // Get the path to the current executable
    let exe = match std::env::current_exe() {
        Ok(path) => path,
        Err(e) => return Response::err(500, &format!("Failed to get executable path: {}", e), ErrorKind::Storage),
    };

    // Spawn the daemon process
    match Command::new(&exe)
        .args(["daemon", "run"])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        Ok(child) => {
            let pid = child.id();
            if let Err(e) = fs::write(&pid_file, pid.to_string()) {
                return Response::err(500, &format!("Failed to write PID file: {}", e), ErrorKind::Storage);
            }

            let config = ServerConfig::default();
            Response::success_with_payload(
                200,
                "Daemon started",
                serde_json::json!({
                    "pid": pid,
                    "socket": config.socket_path.display().to_string(),
                }),
            )
        }
        Err(e) => Response::err(500, &format!("Failed to start daemon: {}", e), ErrorKind::Storage),
    }
}

/// Stop the running daemon
pub async fn daemon_stop() -> Response {
    let pid_file = pid_file_path();

    let pid = match read_pid(&pid_file) {
        Some(p) => p,
        None => return Response::err(404, "Daemon not running (no PID file)", ErrorKind::NotFound),
    };

    if !is_process_running(pid) {
        let _ = fs::remove_file(&pid_file);
        return Response::err(404, "Daemon not running (stale PID file removed)", ErrorKind::NotFound);
    }

    // Send SIGTERM to the process
    #[cfg(unix)]
    {
        unsafe {
            libc::kill(pid as i32, libc::SIGTERM);
        }
    }

    #[cfg(not(unix))]
    {
        let _ = Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/F"])
            .output();
    }

    // Remove PID file
    let _ = fs::remove_file(&pid_file);

    Response::success_with_payload(
        200,
        "Daemon stopped",
        serde_json::json!({ "pid": pid }),
    )
}

/// Check daemon status
pub async fn daemon_status() -> Response {
    let pid_file = pid_file_path();
    let config = ServerConfig::default();

    let pid = read_pid(&pid_file);
    let running = pid.map(is_process_running).unwrap_or(false);
    let socket_exists = config.socket_path.exists();

    Response::success_with_payload(
        200,
        if running { "Daemon running" } else { "Daemon not running" },
        serde_json::json!({
            "running": running,
            "pid": pid,
            "socket": config.socket_path.display().to_string(),
            "socket_exists": socket_exists,
        }),
    )
}

/// Run the daemon in the foreground
pub async fn daemon_run() -> Response {
    let pid_file = pid_file_path();

    // Write our PID
    let pid = std::process::id();
    if let Err(e) = fs::write(&pid_file, pid.to_string()) {
        return Response::err(500, &format!("Failed to write PID file: {}", e), ErrorKind::Storage);
    }

    let config = ServerConfig::default();
    let server = Server::new(config);
    let event_tx = server.event_sender();

    info!(pid = pid, socket = %server.socket_path().display(), "Starting daemon");

    // Spawn the event poller
    let event_poller = EventPoller::new(event_tx);
    tokio::spawn(async move {
        event_poller.run().await;
    });

    // Run the server until interrupted
    tokio::select! {
        result = server.run() => {
            if let Err(e) = result {
                let _ = fs::remove_file(&pid_file);
                return Response::err(500, &format!("Server error: {}", e), ErrorKind::Storage);
            }
        }
        _ = signal::ctrl_c() => {
            info!("Received shutdown signal");
        }
    }

    // Cleanup
    let _ = fs::remove_file(&pid_file);

    Response::success(200, "Daemon stopped")
}

fn read_pid(path: &PathBuf) -> Option<u32> {
    fs::read_to_string(path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
}

fn is_process_running(pid: u32) -> bool {
    #[cfg(unix)]
    {
        // On Unix, sending signal 0 checks if process exists
        unsafe { libc::kill(pid as i32, 0) == 0 }
    }

    #[cfg(not(unix))]
    {
        // On Windows, try to open the process
        use std::process::Command;
        Command::new("tasklist")
            .args(["/FI", &format!("PID eq {}", pid)])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).contains(&pid.to_string()))
            .unwrap_or(false)
    }
}
