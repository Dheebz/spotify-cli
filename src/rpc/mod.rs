//! JSON-RPC daemon module
//!
//! Provides a daemon mode for spotify-cli that exposes JSON-RPC 2.0
//! over Unix sockets, enabling control from editors and other applications.

pub mod dispatch;
pub mod events;
pub mod protocol;
pub mod server;

pub use protocol::{RpcNotification, RpcRequest, RpcResponse};
pub use server::{Server, ServerConfig};
