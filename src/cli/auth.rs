//! Auth command handlers.
use anyhow::bail;
use clap::Subcommand;

use crate::AppContext;
use crate::error::Result;

#[derive(Subcommand, Debug)]
pub enum AuthCommand {
    Login {
        #[arg(long, help = "Spotify client id")]
        client_id: Option<String>,
        #[arg(long, help = "Redirect URI for OAuth")]
        redirect_uri: Option<String>,
    },
    Check,
    Status,
    Scopes,
    Logout,
}

pub fn handle(command: AuthCommand, ctx: &AppContext) -> Result<()> {
    match command {
        AuthCommand::Login {
            client_id,
            redirect_uri,
        } => login(client_id, redirect_uri, ctx),
        AuthCommand::Check => status(ctx),
        AuthCommand::Status => status(ctx),
        AuthCommand::Scopes => scopes(ctx),
        AuthCommand::Logout => logout(ctx),
    }
}

fn login(client_id: Option<String>, redirect_uri: Option<String>, ctx: &AppContext) -> Result<()> {
    let client_id = match client_id.or_else(|| std::env::var("SPOTIFY_CLIENT_ID").ok()) {
        Some(value) => value,
        None => bail!("missing client id; pass --client-id or set SPOTIFY_CLIENT_ID"),
    };

    if let Some(redirect_uri) = redirect_uri {
        ctx.auth.login_oauth_with_redirect(client_id, &redirect_uri)
    } else {
        ctx.auth.login_oauth(client_id)
    }
}

fn status(ctx: &AppContext) -> Result<()> {
    let status = ctx.auth.status()?;
    ctx.output.auth_status(status)
}

fn scopes(ctx: &AppContext) -> Result<()> {
    let scopes = ctx.auth.scopes()?;
    ctx.output.auth_scopes(scopes)
}

fn logout(ctx: &AppContext) -> Result<()> {
    ctx.auth.clear()?;
    Ok(())
}
