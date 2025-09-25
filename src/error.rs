use bevy::log::{debug, error, info, warn};
use chrono::{DateTime, Utc};
use parse_display::Display;
use std::{
    backtrace::Backtrace,
    fmt::{self, Display, Formatter},
};
use uuid::Uuid;

/// Error type for the Dorian game
#[derive(Debug)]
pub struct Error {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub level: ErrorLevel,
    pub message: String,
    pub source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    pub backtrace: Backtrace,
}

/// Error level
#[derive(Display, Debug)]
#[display(style = "UPPERCASE")]
pub enum ErrorLevel {
    Error,
    Warning,
    Info,
    Debug,
}

impl Error {
    // Create a new error
    pub fn new(
        level: ErrorLevel,
        message: &str,
        source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    ) -> Self {
        // Build the error
        let err: Self = Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            level,
            message: message.to_string(),
            source,
            backtrace: Backtrace::force_capture(),
        };

        // Log the error
        err.log();

        err
    }

    // Log the error
    fn log(&self) {
        let err: String = self.to_string();
        match self.level {
            ErrorLevel::Error => error!("{err}"),
            ErrorLevel::Warning => warn!("{err}"),
            ErrorLevel::Info => info!("{err}"),
            ErrorLevel::Debug => debug!("{err}"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{timestamp}] [{level}] [{id}] {message} | Source: {source} | Backtrace: {backtrace}",
            timestamp = self.timestamp.to_rfc3339(),
            id = self.id,
            message = self.message,
            level = self.level,
            source = self
                .source
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or("No source provided".to_string()),
            backtrace = self.backtrace,
        )
    }
}
