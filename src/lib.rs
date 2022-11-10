//! # Usage
//! ```rust
//! use logs::{debug, error, info, trace, warn, Logs};
//!
//! Logs::new().init();
//! trace!("This is a trace log");
//! debug!("This is a debug log");
//! info!("This is a info log");
//! warn!("This is a warn log");
//! error!("This is a error log");
//! ```
//! Output:
//! ```ignore
//! 2022-09-06T08:38:23.490 [TRACE] This is a trace log
//! 2022-09-06T08:38:23.490 [DEBUG] This is a debug log
//! 2022-09-06T08:38:23.490 [INFO ] This is a info log
//! 2022-09-06T08:38:23.490 [WARN ] This is a warn log
//! 2022-09-06T08:38:23.490 [ERROR] This is a error log
//! ```
//! ## Options
//! ```ignore
//! use logs::{Logs, debug, error, LevelFilter};
//! Logs::new()
//!     // Show log level color
//!     .color(true)
//!     // Filter log target
//!     .target("target")
//!     // Filter log level
//!     .level(LevelFilter::Info)
//!     // Filter log target from `LOG` environment variable
//!     .level_from_default_env()
//!     .unwrap()
//!     // Filter log target from `NAME` environment variable
//!     .level_from_env("NAME")
//!     .unwrap()
//!     // Filter log level from str
//!     .level_from_str("info")
//!     .unwrap()
//!     // Apply
//!     .init();
//! ```

pub use log::{debug, error, info, trace, warn, LevelFilter};
use log::{Level, Log, Metadata, ParseLevelError, Record};
use std::{
    env::{self, VarError},
    str::FromStr,
};
use time::{format_description::FormatItem, OffsetDateTime};

const TIMESTAMP_FORMAT_OFFSET: &[FormatItem] = time::macros::format_description!(
    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]"
);

/// Logger config
#[derive(Debug, Clone)]
pub struct Logs {
    level: LevelFilter,
    color: bool,
    target: Option<String>,
}

#[derive(Debug)]
pub enum LogsError {
    Level(ParseLevelError),
    Env(VarError),
}

impl Default for Logs {
    fn default() -> Self {
        Self::new()
    }
}

impl Logs {
    /// Create a new logs
    pub fn new() -> Self {
        Self {
            level: LevelFilter::Trace,
            color: false,
            target: None,
        }
    }

    /// Set log level color
    pub fn color(mut self, color: bool) -> Self {
        self.color = color;
        self
    }

    /// Filter log target
    pub fn target<S: AsRef<str>>(mut self, target: S) -> Self {
        let target = target.as_ref().replace('-', "_");
        self.target = Some(target);
        self
    }

    /// Filter log level
    pub fn level(mut self, level: LevelFilter) -> Self {
        self.level = level;
        self
    }

    /// Filter log level from `LOG` environment variable
    pub fn level_from_default_env(self) -> Result<Self, LogsError> {
        self.level_from_env("LOG")
    }

    /// Filter log level from `NAME` environment variable
    pub fn level_from_env<S: AsRef<str>>(self, name: S) -> Result<Self, LogsError> {
        match env::var(name.as_ref()) {
            Ok(s) => self.level_from_str(&s),
            Err(err) => Err(LogsError::Env(err)),
        }
    }

    /// Filter log level from `str`
    pub fn level_from_str<S: AsRef<str>>(mut self, s: S) -> Result<Self, LogsError> {
        match LevelFilter::from_str(s.as_ref()) {
            Ok(level) => {
                self.level = level;
                Ok(self)
            }
            Err(err) => Err(LogsError::Level(err)),
        }
    }

    /// log init
    pub fn init(self) {
        let rst =
            log::set_boxed_logger(Box::new(self)).map(|()| log::set_max_level(LevelFilter::Trace));
        if let Err(err) = rst {
            panic!("log config failed {:#?}", err);
        }
    }
}

impl Log for Logs {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if let Some(level) = self.level.to_level() {
            if level >= metadata.level() {
                return match &self.target {
                    Some(t) => metadata.target().starts_with(t),
                    None => true,
                };
            }
        }
        false
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let datetime = OffsetDateTime::now_utc()
                .format(&TIMESTAMP_FORMAT_OFFSET)
                .unwrap();
            let level = level_to_str(record.level(), self.color);

            println!("{} [{}] {}", datetime, level, record.args());
        }
    }

    fn flush(&self) {}
}

fn level_to_str(level: Level, color: bool) -> &'static str {
    if color {
        match level {
            Level::Error => "\x1B[31mERROR\x1B[0m",
            Level::Warn => "\x1B[33mWARN \x1B[0m",
            Level::Info => "\x1B[32mINFO \x1B[0m",
            Level::Debug => "\x1B[3;34mDEBUG\x1B[0m",
            Level::Trace => "\x1B[2;3mTRACE\x1B[0m",
        }
    } else {
        match level {
            Level::Error => "ERROR",
            Level::Warn => "WARN ",
            Level::Info => "INFO ",
            Level::Debug => "DEBUG",
            Level::Trace => "TRACE",
        }
    }
}
