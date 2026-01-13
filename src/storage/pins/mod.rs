mod fuzzy;
mod pin;
mod resource_type;
mod store;

pub use pin::Pin;
pub use resource_type::ResourceType;
pub use store::{PinError, PinStore};

const PINS_FILE: &str = "pins.json";
