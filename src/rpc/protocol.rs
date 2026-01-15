//! JSON-RPC 2.0 protocol types
//!
//! Implements the JSON-RPC 2.0 specification for request/response/notification.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::io::output::Response;

/// JSON-RPC 2.0 request
#[derive(Debug, Clone, Deserialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default)]
    pub params: Option<Value>,
    pub id: Option<Value>,
}

impl RpcRequest {
    /// Check if this is a notification (no id = no response expected)
    pub fn is_notification(&self) -> bool {
        self.id.is_none()
    }
}

/// JSON-RPC 2.0 response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
    pub id: Value,
}

impl RpcResponse {
    /// Create a success response
    pub fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id,
        }
    }

    /// Create an error response
    pub fn error(id: Value, code: i32, message: &str, data: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(RpcError {
                code,
                message: message.to_string(),
                data,
            }),
            id,
        }
    }

    /// Create from CLI Response type
    pub fn from_response(id: Value, response: Response) -> Self {
        match response.status {
            crate::io::output::Status::Success => {
                let result = serde_json::json!({
                    "message": response.message,
                    "payload": response.payload,
                });
                Self::success(id, result)
            }
            crate::io::output::Status::Error => {
                let data = response.error.map(|e| {
                    serde_json::json!({
                        "kind": format!("{:?}", e.kind),
                        "details": e.details,
                    })
                });
                Self::error(id, response.code as i32, &response.message, data)
            }
        }
    }
}

/// JSON-RPC 2.0 error object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// JSON-RPC 2.0 notification (server → client, no id)
#[derive(Debug, Clone, Serialize)]
pub struct RpcNotification {
    pub jsonrpc: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl RpcNotification {
    pub fn new(method: &str, params: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
        }
    }
}

/// Standard JSON-RPC error codes
pub mod error_codes {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const INTERNAL_ERROR: i32 = -32603;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_request_with_params() {
        let json = r#"{"jsonrpc": "2.0", "method": "player.play", "params": {"uri": "spotify:track:123"}, "id": 1}"#;
        let req: RpcRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.method, "player.play");
        assert!(req.params.is_some());
        assert!(!req.is_notification());
    }

    #[test]
    fn parse_notification() {
        let json = r#"{"jsonrpc": "2.0", "method": "player.next"}"#;
        let req: RpcRequest = serde_json::from_str(json).unwrap();
        assert!(req.is_notification());
    }

    #[test]
    fn serialize_success_response() {
        let resp = RpcResponse::success(serde_json::json!(1), serde_json::json!({"status": "ok"}));
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("result"));
        assert!(!json.contains("error"));
    }

    #[test]
    fn serialize_error_response() {
        let resp = RpcResponse::error(serde_json::json!(1), -32601, "Method not found", None);
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("error"));
        assert!(!json.contains("result"));
    }

    #[test]
    fn serialize_notification() {
        let notif = RpcNotification::new("event.trackChanged", Some(serde_json::json!({"track": "test"})));
        let json = serde_json::to_string(&notif).unwrap();
        assert!(json.contains("event.trackChanged"));
        assert!(!json.contains("id"));
    }
}
