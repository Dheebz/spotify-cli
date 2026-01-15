//! Daemon command arguments

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum DaemonCommand {
    /// Start the daemon in the background
    Start,

    /// Stop the running daemon
    Stop,

    /// Check if the daemon is running
    Status,

    /// Run the daemon in the foreground (for debugging/systemd)
    Run,
}
