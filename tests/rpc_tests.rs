//! RPC integration tests
//!
//! Tests for the JSON-RPC server and protocol implementation.

use spotify_cli::rpc::protocol::{RpcNotification, RpcRequest, RpcResponse, error_codes};
use spotify_cli::rpc::{Server, ServerConfig};
use std::path::PathBuf;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tempfile::{tempdir, TempDir};

/// Helper to create a test server with a temporary socket
/// Returns TempDir to keep it alive for the duration of the test
async fn setup_test_server() -> (Server, PathBuf, TempDir) {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let socket_path = temp_dir.path().join("test.sock");

    let config = ServerConfig {
        socket_path: socket_path.clone(),
    };

    let server = Server::new(config);
    (server, socket_path, temp_dir)
}

/// Helper to send a JSON-RPC request and get response
async fn send_request(socket_path: &PathBuf, request: &str) -> String {
    let stream = UnixStream::connect(socket_path).await.expect("Failed to connect");
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    writer.write_all(request.as_bytes()).await.expect("Failed to write");
    writer.write_all(b"\n").await.expect("Failed to write newline");
    writer.flush().await.expect("Failed to flush");

    let mut response = String::new();
    reader.read_line(&mut response).await.expect("Failed to read");
    response
}

// ============================================================
// Protocol Tests
// ============================================================

#[test]
fn test_rpc_request_parsing() {
    let json = r#"{"jsonrpc":"2.0","method":"ping","id":1}"#;
    let req: RpcRequest = serde_json::from_str(json).expect("Failed to parse request");

    assert_eq!(req.jsonrpc, "2.0");
    assert_eq!(req.method, "ping");
    assert!(!req.is_notification());
}

#[test]
fn test_rpc_request_with_params() {
    let json = r#"{"jsonrpc":"2.0","method":"player.play","params":{"uri":"spotify:track:123"},"id":2}"#;
    let req: RpcRequest = serde_json::from_str(json).expect("Failed to parse request");

    assert_eq!(req.method, "player.play");
    assert!(req.params.is_some());
    let params = req.params.unwrap();
    assert_eq!(params["uri"], "spotify:track:123");
}

#[test]
fn test_rpc_notification_no_id() {
    let json = r#"{"jsonrpc":"2.0","method":"player.next"}"#;
    let req: RpcRequest = serde_json::from_str(json).expect("Failed to parse notification");

    assert!(req.is_notification());
    assert!(req.id.is_none());
}

#[test]
fn test_rpc_response_success_serialization() {
    let resp = RpcResponse::success(
        serde_json::json!(1),
        serde_json::json!({"status": "ok"}),
    );
    let json = serde_json::to_string(&resp).expect("Failed to serialize");

    assert!(json.contains(r#""jsonrpc":"2.0""#));
    assert!(json.contains(r#""result""#));
    assert!(json.contains(r#""id":1"#));
    assert!(!json.contains(r#""error""#));
}

#[test]
fn test_rpc_response_error_serialization() {
    let resp = RpcResponse::error(
        serde_json::json!(1),
        error_codes::METHOD_NOT_FOUND,
        "Method not found",
        None,
    );
    let json = serde_json::to_string(&resp).expect("Failed to serialize");

    assert!(json.contains(r#""error""#));
    assert!(json.contains(r#"-32601"#));
    assert!(!json.contains(r#""result""#));
}

#[test]
fn test_rpc_notification_serialization() {
    let notif = RpcNotification::new(
        "event.trackChanged",
        Some(serde_json::json!({"track_id": "abc123"})),
    );
    let json = serde_json::to_string(&notif).expect("Failed to serialize");

    assert!(json.contains(r#""method":"event.trackChanged""#));
    assert!(json.contains(r#""params""#));
    assert!(!json.contains(r#""id""#));
}

#[test]
fn test_rpc_notification_without_params() {
    let notif = RpcNotification::new("event.playbackPaused", None);
    let json = serde_json::to_string(&notif).expect("Failed to serialize");

    assert!(json.contains(r#""method":"event.playbackPaused""#));
    assert!(!json.contains(r#""params""#));
}

// ============================================================
// Error Code Tests
// ============================================================

#[test]
fn test_error_codes_values() {
    assert_eq!(error_codes::PARSE_ERROR, -32700);
    assert_eq!(error_codes::INVALID_REQUEST, -32600);
    assert_eq!(error_codes::METHOD_NOT_FOUND, -32601);
    assert_eq!(error_codes::INVALID_PARAMS, -32602);
    assert_eq!(error_codes::INTERNAL_ERROR, -32603);
}

// ============================================================
// Server Configuration Tests
// ============================================================

#[test]
fn test_server_config_default() {
    let config = ServerConfig::default();
    assert!(config.socket_path.to_string_lossy().contains("daemon.sock"));
}

#[test]
fn test_server_config_custom_path() {
    let config = ServerConfig {
        socket_path: PathBuf::from("/tmp/custom.sock"),
    };
    assert_eq!(config.socket_path, PathBuf::from("/tmp/custom.sock"));
}

// ============================================================
// Server Tests
// ============================================================

#[tokio::test]
async fn test_server_socket_path() {
    let (server, socket_path, _temp_dir) = setup_test_server().await;
    assert_eq!(server.socket_path(), &socket_path);
}

#[tokio::test]
async fn test_server_event_sender() {
    let (server, _socket_path, _temp_dir) = setup_test_server().await;
    let tx = server.event_sender();

    // Should be able to send notifications
    let notif = RpcNotification::new("test.event", None);
    let result = tx.send(notif);
    // No receivers, so this returns Err, but that's expected
    assert!(result.is_err());
}

// ============================================================
// Integration Tests (require running server)
// ============================================================

#[tokio::test]
async fn test_server_ping_pong() {
    let (server, socket_path, _temp_dir) = setup_test_server().await;

    // Start server in background
    let server_handle = tokio::spawn(async move {
        let _ = server.run().await;
    });

    // Give server time to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Send ping request
    let request = r#"{"jsonrpc":"2.0","method":"ping","id":1}"#;
    let response = send_request(&socket_path, request).await;

    // Parse and verify response
    let resp: RpcResponse = serde_json::from_str(&response).expect("Failed to parse response");
    assert!(resp.result.is_some());
    assert!(resp.error.is_none());

    let result = resp.result.unwrap();
    assert_eq!(result["message"], "pong");

    // Cleanup
    server_handle.abort();
}

#[tokio::test]
async fn test_server_version() {
    let (server, socket_path, _temp_dir) = setup_test_server().await;

    let server_handle = tokio::spawn(async move {
        let _ = server.run().await;
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let request = r#"{"jsonrpc":"2.0","method":"version","id":2}"#;
    let response = send_request(&socket_path, request).await;

    let resp: RpcResponse = serde_json::from_str(&response).expect("Failed to parse response");
    assert!(resp.result.is_some());

    let result = resp.result.unwrap();
    assert!(result["payload"]["version"].is_string());
    assert!(result["payload"]["name"].is_string());

    server_handle.abort();
}

#[tokio::test]
async fn test_server_unknown_method() {
    let (server, socket_path, _temp_dir) = setup_test_server().await;

    let server_handle = tokio::spawn(async move {
        let _ = server.run().await;
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let request = r#"{"jsonrpc":"2.0","method":"unknown.method","id":3}"#;
    let response = send_request(&socket_path, request).await;

    let resp: RpcResponse = serde_json::from_str(&response).expect("Failed to parse response");
    assert!(resp.error.is_some());
    assert!(resp.result.is_none());

    let error = resp.error.unwrap();
    // Response uses HTTP-style 404 code for "not found"
    assert_eq!(error.code, 404);

    server_handle.abort();
}

#[tokio::test]
async fn test_server_parse_error() {
    let (server, socket_path, _temp_dir) = setup_test_server().await;

    let server_handle = tokio::spawn(async move {
        let _ = server.run().await;
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Send malformed JSON
    let request = r#"{"invalid json"#;
    let response = send_request(&socket_path, request).await;

    let resp: RpcResponse = serde_json::from_str(&response).expect("Failed to parse response");
    assert!(resp.error.is_some());

    let error = resp.error.unwrap();
    assert_eq!(error.code, error_codes::PARSE_ERROR);

    server_handle.abort();
}

#[tokio::test]
async fn test_server_multiple_requests() {
    let (server, socket_path, _temp_dir) = setup_test_server().await;

    let server_handle = tokio::spawn(async move {
        let _ = server.run().await;
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Send multiple requests on same connection
    let stream = UnixStream::connect(&socket_path).await.expect("Failed to connect");
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    // First request
    writer.write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"ping\",\"id\":1}\n").await.unwrap();
    writer.flush().await.unwrap();

    let mut response1 = String::new();
    reader.read_line(&mut response1).await.unwrap();
    let resp1: RpcResponse = serde_json::from_str(&response1).unwrap();
    assert!(resp1.result.is_some());

    // Second request
    writer.write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"version\",\"id\":2}\n").await.unwrap();
    writer.flush().await.unwrap();

    let mut response2 = String::new();
    reader.read_line(&mut response2).await.unwrap();
    let resp2: RpcResponse = serde_json::from_str(&response2).unwrap();
    assert!(resp2.result.is_some());

    server_handle.abort();
}

#[tokio::test]
async fn test_server_notification_no_response() {
    let (server, socket_path, _temp_dir) = setup_test_server().await;

    let server_handle = tokio::spawn(async move {
        let _ = server.run().await;
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let stream = UnixStream::connect(&socket_path).await.expect("Failed to connect");
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    // Send notification (no id)
    writer.write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"ping\"}\n").await.unwrap();
    writer.flush().await.unwrap();

    // Immediately send a regular request
    writer.write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"ping\",\"id\":1}\n").await.unwrap();
    writer.flush().await.unwrap();

    // Should only get one response (for the regular request)
    let mut response = String::new();
    reader.read_line(&mut response).await.unwrap();
    let resp: RpcResponse = serde_json::from_str(&response).unwrap();

    // The response should be for id:1, not the notification
    assert_eq!(resp.id, serde_json::json!(1));

    server_handle.abort();
}

// ============================================================
// Response Conversion Tests
// ============================================================

#[test]
fn test_rpc_response_from_success() {
    use spotify_cli::io::output::Response;

    let cli_response = Response::success(200, "Success message");
    let rpc_response = RpcResponse::from_response(serde_json::json!(1), cli_response);

    assert!(rpc_response.result.is_some());
    assert!(rpc_response.error.is_none());

    let result = rpc_response.result.unwrap();
    assert_eq!(result["message"], "Success message");
}

#[test]
fn test_rpc_response_from_success_with_payload() {
    use spotify_cli::io::output::Response;

    let cli_response = Response::success_with_payload(
        200,
        "Data retrieved",
        serde_json::json!({"key": "value"}),
    );
    let rpc_response = RpcResponse::from_response(serde_json::json!(2), cli_response);

    assert!(rpc_response.result.is_some());
    let result = rpc_response.result.unwrap();
    assert_eq!(result["payload"]["key"], "value");
}

#[test]
fn test_rpc_response_from_error() {
    use spotify_cli::io::output::{ErrorKind, Response};

    let cli_response = Response::err(404, "Not found", ErrorKind::NotFound);
    let rpc_response = RpcResponse::from_response(serde_json::json!(3), cli_response);

    assert!(rpc_response.error.is_some());
    assert!(rpc_response.result.is_none());

    let error = rpc_response.error.unwrap();
    assert_eq!(error.code, 404);
    assert_eq!(error.message, "Not found");
}
