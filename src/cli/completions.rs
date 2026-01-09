//! Completion script output and installation.
use clap::Args;
use std::path::PathBuf;
use std::{env, fs};

use crate::error::Result;

#[derive(Args, Debug)]
pub struct CompletionsCommand {
    #[arg(value_name = "SHELL", value_parser = ["bash", "zsh", "fish"])]
    shell: String,
    #[arg(long, help = "Install to the default completion directory")]
    install: bool,
}

pub fn handle(command: CompletionsCommand) -> Result<()> {
    if command.install {
        return install(&command.shell);
    }

    match command.shell.as_str() {
        "bash" => print_script(include_str!("../../completions/spotify.bash")),
        "zsh" => print_script(include_str!("../../completions/_spotify-cli")),
        "fish" => print_script(include_str!("../../completions/spotify.fish")),
        _ => Ok(()),
    }
}

fn print_script(script: &str) -> Result<()> {
    print!("{script}");
    Ok(())
}

fn install(shell: &str) -> Result<()> {
    let home = env::var("HOME").unwrap_or_default();
    let (relative, script) = match shell {
        "bash" => (
            ".bash_completion.d/spotify-cli",
            include_str!("../../completions/spotify.bash"),
        ),
        "zsh" => (
            ".zsh/completions/_spotify-cli",
            include_str!("../../completions/_spotify-cli"),
        ),
        "fish" => (
            ".config/fish/completions/spotify-cli.fish",
            include_str!("../../completions/spotify.fish"),
        ),
        _ => return Ok(()),
    };

    let path = PathBuf::from(home).join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, script)?;
    println!("{}", path.display());
    Ok(())
}
