use std::process::Command;

fn bin() -> String {
    let exe = std::env::current_exe().expect("current exe");
    let target_dir = exe
        .parent()
        .and_then(|path| path.parent())
        .expect("target dir");
    let bin_name = if cfg!(windows) {
        "spotify-cli.exe"
    } else {
        "spotify-cli"
    };
    let bin = target_dir.join(bin_name);
    bin.to_string_lossy().to_string()
}

fn temp_dir(name: &str) -> std::path::PathBuf {
    let mut path = std::env::temp_dir();
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    path.push(format!("spotify-cli-{name}-{stamp}"));
    std::fs::create_dir_all(&path).unwrap();
    path
}

#[test]
fn help_runs() {
    let output = Command::new(bin()).arg("help").output().expect("run help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("spotify-cli"));
}

#[test]
#[cfg(not(windows))]
fn completions_install_writes_file() {
    let home = temp_dir("home");
    let output = Command::new(bin())
        .arg("completions")
        .arg("zsh")
        .arg("--install")
        .env("HOME", &home)
        .output()
        .expect("run completions install");
    assert!(output.status.success());
    let path = home.join(".zsh").join("completions").join("_spotify-cli");
    assert!(path.exists());
}

#[test]
fn pin_list_smoke() {
    let cache_dir = temp_dir("cache");
    let output = Command::new(bin())
        .arg("pin")
        .arg("list")
        .env("SPOTIFY_CLI_CACHE_DIR", &cache_dir)
        .output()
        .expect("run pin list");
    assert!(output.status.success());
}
