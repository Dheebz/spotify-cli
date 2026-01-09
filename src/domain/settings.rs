use serde::{Deserialize, Serialize};

/// User settings stored in the metadata cache.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Settings {
    pub country: Option<String>,
    pub user_name: Option<String>,
}
