//! Device command handlers.
use anyhow::bail;
use clap::Subcommand;

use crate::AppContext;
use crate::domain::device::Device;
use crate::error::Result;

#[derive(Subcommand, Debug)]
pub enum DeviceCommand {
    List {
        #[arg(long, help = "Query Spotify directly instead of cache")]
        live: bool,
    },
    Set {
        name: String,
    },
}

pub fn handle(command: DeviceCommand, ctx: &AppContext) -> Result<()> {
    match command {
        DeviceCommand::List { live } => list(ctx, live),
        DeviceCommand::Set { name } => set(ctx, &name),
    }
}

fn list(ctx: &AppContext, live: bool) -> Result<()> {
    if live {
        let devices = ctx.spotify()?.devices().list()?;
        return ctx.output.device_list(devices);
    }

    let snapshot = ctx.cache.device_cache().load()?;
    let Some(snapshot) = snapshot else {
        bail!("device cache empty; run `spotify sync`");
    };
    ctx.output.device_list(snapshot.items)
}

fn set(ctx: &AppContext, name: &str) -> Result<()> {
    let snapshot = ctx.cache.device_cache().load()?;
    let Some(snapshot) = snapshot else {
        bail!("device cache empty; run `spotify sync`");
    };

    let matches = find_devices(&snapshot.items, name);
    if matches.is_empty() {
        bail!("no device matches '{name}'");
    }
    if matches.len() > 1 {
        let names: Vec<String> = matches.iter().map(|device| device.name.clone()).collect();
        bail!("multiple devices match: {}", names.join(", "));
    }

    let device = matches[0];
    ctx.spotify()?.devices().set_active(&device.id)?;
    let message = format!("Switched device: {}", device.name);
    ctx.output.action("device_set", &message)
}

fn find_devices<'a>(devices: &'a [Device], query: &str) -> Vec<&'a Device> {
    let query = query.to_lowercase();
    devices
        .iter()
        .filter(|device| device.name.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::find_devices;
    use crate::domain::device::Device;

    #[test]
    fn find_devices_matches_case_insensitive() {
        let devices = vec![
            Device {
                id: "1".to_string(),
                name: "Office Speaker".to_string(),
                volume_percent: Some(50),
            },
            Device {
                id: "2".to_string(),
                name: "Phone".to_string(),
                volume_percent: None,
            },
        ];

        let matches = find_devices(&devices, "office");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].id, "1");
    }
}
