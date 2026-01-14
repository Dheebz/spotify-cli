//! Structured logging configuration.
//!
//! Supports both human-readable and JSON output formats with configurable verbosity.

use tracing_subscriber::{fmt, EnvFilter};

/// Log output format.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LogFormat {
    /// Human-readable colored output (default)
    #[default]
    Pretty,
    /// JSON structured output for machine parsing
    Json,
}

impl std::str::FromStr for LogFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pretty" | "text" | "human" => Ok(LogFormat::Pretty),
            "json" => Ok(LogFormat::Json),
            _ => Err(format!("Invalid log format: {}. Use 'pretty' or 'json'", s)),
        }
    }
}

impl std::fmt::Display for LogFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogFormat::Pretty => write!(f, "pretty"),
            LogFormat::Json => write!(f, "json"),
        }
    }
}

/// Logging configuration.
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// Verbosity level (0=warn, 1=info, 2=debug, 3+=trace)
    pub verbosity: u8,
    /// Output format
    pub format: LogFormat,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            verbosity: 0,
            format: LogFormat::Pretty,
        }
    }
}

impl LogConfig {
    /// Create a new log config with given verbosity.
    pub fn new(verbosity: u8) -> Self {
        Self {
            verbosity,
            format: LogFormat::Pretty,
        }
    }

    /// Set the output format.
    pub fn format(mut self, format: LogFormat) -> Self {
        self.format = format;
        self
    }

    /// Get the tracing filter level based on verbosity.
    fn filter(&self) -> EnvFilter {
        let level = match self.verbosity {
            0 => "warn",
            1 => "info",
            2 => "debug",
            _ => "trace",
        };
        EnvFilter::new(level)
    }

    /// Initialize the global tracing subscriber.
    ///
    /// Should be called once at application startup.
    pub fn init(self) {
        let filter = self.filter();

        match self.format {
            LogFormat::Pretty => {
                fmt()
                    .with_env_filter(filter)
                    .with_target(false)
                    .without_time()
                    .init();
            }
            LogFormat::Json => {
                fmt()
                    .with_env_filter(filter)
                    .json()
                    .with_current_span(true)
                    .init();
            }
        }
    }
}

/// Helper macro for structured command logging.
///
/// Use at the start of command handlers to log command execution.
#[macro_export]
macro_rules! log_command {
    ($cmd:expr) => {
        tracing::info!(command = $cmd, "Executing command");
    };
    ($cmd:expr, $($field:tt)*) => {
        tracing::info!(command = $cmd, $($field)*, "Executing command");
    };
}

/// Helper macro for logging command completion with timing.
#[macro_export]
macro_rules! log_command_complete {
    ($cmd:expr, $duration_ms:expr) => {
        tracing::info!(command = $cmd, duration_ms = $duration_ms, "Command completed");
    };
    ($cmd:expr, $duration_ms:expr, $($field:tt)*) => {
        tracing::info!(command = $cmd, duration_ms = $duration_ms, $($field)*, "Command completed");
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_format_from_str() {
        assert_eq!("pretty".parse::<LogFormat>().unwrap(), LogFormat::Pretty);
        assert_eq!("text".parse::<LogFormat>().unwrap(), LogFormat::Pretty);
        assert_eq!("human".parse::<LogFormat>().unwrap(), LogFormat::Pretty);
        assert_eq!("json".parse::<LogFormat>().unwrap(), LogFormat::Json);
        assert_eq!("JSON".parse::<LogFormat>().unwrap(), LogFormat::Json);
        assert!("invalid".parse::<LogFormat>().is_err());
    }

    #[test]
    fn log_format_display() {
        assert_eq!(LogFormat::Pretty.to_string(), "pretty");
        assert_eq!(LogFormat::Json.to_string(), "json");
    }

    #[test]
    fn log_config_builder() {
        let config = LogConfig::new(2).format(LogFormat::Json);
        assert_eq!(config.verbosity, 2);
        assert_eq!(config.format, LogFormat::Json);
    }

    #[test]
    fn log_config_default() {
        let config = LogConfig::default();
        assert_eq!(config.verbosity, 0);
        assert_eq!(config.format, LogFormat::Pretty);
    }
}
