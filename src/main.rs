mod action;
mod cache;
mod cli;
mod domain;
mod error;
mod output;
mod spotify;

use crate::cache::Cache;
use crate::error::Result;
use crate::output::Output;
use crate::spotify::auth::AuthService;
use crate::spotify::client::SpotifyClient;
use anyhow::Error;
use std::sync::OnceLock;

/// Shared runtime context for command handlers.
pub struct AppContext {
    pub cache: Cache,
    pub auth: AuthService,
    pub output: Output,
    pub verbose: bool,
    spotify: OnceLock<Result<SpotifyClient>>,
}

fn main() -> Result<()> {
    let parsed = cli::parse();

    let cache = Cache::new()?;
    cache.ensure_dirs()?;

    let auth = AuthService::new(cache.metadata_store());
    let output = Output::new(parsed.json, auth.user_name()?, None, false);

    let ctx = AppContext {
        cache,
        auth,
        output,
        verbose: false,
        spotify: OnceLock::new(),
    };

    cli::execute(parsed, &ctx)
}

impl AppContext {
    pub fn spotify(&self) -> Result<&SpotifyClient> {
        let client = self
            .spotify
            .get_or_init(|| SpotifyClient::new(self.auth.clone()));
        match client {
            Ok(client) => Ok(client),
            Err(err) => Err(Error::msg(err.to_string())),
        }
    }
}
