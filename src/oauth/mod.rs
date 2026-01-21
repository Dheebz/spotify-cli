//! OAuth 2.0 authentication with PKCE for Spotify Web API.
//!
//! This module implements the Authorization Code flow with PKCE (Proof Key for Code Exchange),
//! which is the recommended flow for public clients like CLI applications.
//!
//! ## Flow Overview
//!
//! 1. Generate PKCE challenge/verifier pair
//! 2. Open browser to Spotify authorization URL
//! 3. Start local callback server to receive auth code
//! 4. Exchange auth code for access/refresh tokens
//!
//! ## Usage
//!
//! ```ignore
//! use spotify_cli::oauth::flow::OAuthFlow;
//!
//! let flow = OAuthFlow::new("client_id".to_string());
//! let token = flow.authenticate().await?;
//! ```

pub mod callback_server;
pub mod flow;
pub mod pkce;
pub mod token;
