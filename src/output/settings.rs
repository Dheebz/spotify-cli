//! Settings output formatting.
use serde::Serialize;

use crate::domain::settings::Settings;
use crate::error::Result;

pub fn settings_human(settings: Settings) -> Result<()> {
    if let Some(country) = settings.country {
        println!("country={}", country);
    }
    if let Some(user_name) = settings.user_name {
        println!("user_name={}", user_name);
    }
    Ok(())
}

#[derive(Serialize)]
struct SettingsPayload {
    country: Option<String>,
    user_name: Option<String>,
}

pub fn settings_json(settings: Settings) -> Result<()> {
    let payload = settings_payload(settings);
    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

fn settings_payload(settings: Settings) -> SettingsPayload {
    SettingsPayload {
        country: settings.country,
        user_name: settings.user_name,
    }
}

#[cfg(test)]
mod tests {
    use super::settings_payload;
    use crate::domain::settings::Settings;

    #[test]
    fn settings_payload_shape() {
        let payload = settings_payload(Settings {
            country: Some("AU".to_string()),
            user_name: None,
        });
        assert_eq!(payload.country.as_deref(), Some("AU"));
        assert!(payload.user_name.is_none());
    }
}
