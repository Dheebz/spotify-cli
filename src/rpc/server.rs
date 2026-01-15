//! Unix socket server for JSON-RPC
//!
//! Handles client connections, parses JSON-RPC requests, and dispatches to handlers.

use std::path::PathBuf;
use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};

use crate::storage::paths;

use super::dispatch::Dispatcher;
use super::protocol::{error_codes, RpcNotification, RpcRequest, RpcResponse};

/// RPC Server configuration
pub struct ServerConfig {
    pub socket_path: PathBuf,
}

impl Default for ServerConfig {
    fn default() -> Self {
        let socket_path = paths::socket_file()
            .unwrap_or_else(|_| PathBuf::from("/tmp/spotify-cli.sock"));

        Self { socket_path }
    }
}

/// RPC Server
pub struct Server {
    config: ServerConfig,
    dispatcher: Arc<Dispatcher>,
    event_tx: broadcast::Sender<RpcNotification>,
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        let (event_tx, _) = broadcast::channel(100);

        Self {
            config,
            dispatcher: Arc::new(Dispatcher::new()),
            event_tx,
        }
    }

    /// Get the socket path
    pub fn socket_path(&self) -> &PathBuf {
        &self.config.socket_path
    }

    /// Get event broadcaster for sending notifications
    pub fn event_sender(&self) -> broadcast::Sender<RpcNotification> {
        self.event_tx.clone()
    }

    /// Run the server
    pub async fn run(&self) -> std::io::Result<()> {
        // Remove existing socket file
        if self.config.socket_path.exists() {
            std::fs::remove_file(&self.config.socket_path)?;
        }

        let listener = UnixListener::bind(&self.config.socket_path)?;
        info!(path = %self.config.socket_path.display(), "RPC server listening");

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let dispatcher = Arc::clone(&self.dispatcher);
                    let event_rx = self.event_tx.subscribe();

                    tokio::spawn(async move {
                        if let Err(e) = handle_client(stream, dispatcher, event_rx).await {
                            debug!(error = %e, "Client connection ended");
                        }
                    });
                }
                Err(e) => {
                    error!(error = %e, "Failed to accept connection");
                }
            }
        }
    }
}

/// Handle a single client connection
async fn handle_client(
    stream: UnixStream,
    dispatcher: Arc<Dispatcher>,
    mut event_rx: broadcast::Receiver<RpcNotification>,
) -> std::io::Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    debug!("Client connected");

    loop {
        line.clear();

        tokio::select! {
            // Handle incoming requests
            result = reader.read_line(&mut line) => {
                match result {
                    Ok(0) => {
                        debug!("Client disconnected");
                        break;
                    }
                    Ok(_) => {
                        let response = process_request(&line, &dispatcher).await;
                        if let Some(resp) = response {
                            let json = serde_json::to_string(&resp).unwrap_or_default();
                            writer.write_all(json.as_bytes()).await?;
                            writer.write_all(b"\n").await?;
                            writer.flush().await?;
                        }
                    }
                    Err(e) => {
                        warn!(error = %e, "Read error");
                        break;
                    }
                }
            }

            // Forward event notifications to client
            result = event_rx.recv() => {
                match result {
                    Ok(notification) => {
                        let json = serde_json::to_string(&notification).unwrap_or_default();
                        if writer.write_all(json.as_bytes()).await.is_err() {
                            break;
                        }
                        if writer.write_all(b"\n").await.is_err() {
                            break;
                        }
                        let _ = writer.flush().await;
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        // Client is too slow, skip some events
                        continue;
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

/// Process a single JSON-RPC request
async fn process_request(line: &str, dispatcher: &Dispatcher) -> Option<RpcResponse> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    // Parse the request
    let request: RpcRequest = match serde_json::from_str(line) {
        Ok(req) => req,
        Err(e) => {
            return Some(RpcResponse::error(
                serde_json::Value::Null,
                error_codes::PARSE_ERROR,
                &format!("Parse error: {}", e),
                None,
            ));
        }
    };

    // Notifications don't get responses
    if request.is_notification() {
        let _ = dispatcher.dispatch(&request).await;
        return None;
    }

    // Get the request id
    let id = request.id.clone().unwrap_or(serde_json::Value::Null);

    // Dispatch and return response
    let response = dispatcher.dispatch(&request).await;
    Some(RpcResponse::from_response(id, response))
}

impl Drop for Server {
    fn drop(&mut self) {
        // Clean up socket file
        if self.config.socket_path.exists() {
            let _ = std::fs::remove_file(&self.config.socket_path);
        }
    }
}
