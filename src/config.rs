use std::env::VarError;
use std::str::FromStr;

// Default date format: 0000-00-00 00:00:00
const DEFAULT_DATE_FORMAT: &'static str = "%F %T";

#[doc(hidden)]
pub static mut _LOG_CONFIG: LogConfig = LogConfig::enable_all();

/// Configuration Log
#[derive(Debug, Clone, Copy)]
pub struct LogConfig {
    date_format: &'static str,
    color: bool,
    trace: bool,
    debug: bool,
    info: bool,
    warn: bool,
    error: bool,
}

/// Log Error
#[derive(Debug, Clone)]
pub enum LogError {
    NotPresent,
    NotUnicode,
    FormatError(String),
}

impl Default for LogConfig {
    fn default() -> Self {
        Self::enable_all()
    }
}

impl FromStr for LogConfig {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let mut config = Self::disable_all();
        for item in s.split(',') {
            match item.trim() {
                "all" => {
                    config = Self::enable_all();
                }
                "!all" => {
                    config = Self::disable_all();
                }
                "trace" => {
                    config.trace = true;
                }
                "!trace" => {
                    config.trace = false;
                }
                "debug" => {
                    config.debug = true;
                }
                "!debug" => {
                    config.debug = false;
                }
                "info" => {
                    config.info = true;
                }
                "!info" => {
                    config.info = false;
                }
                "warn" => {
                    config.warn = true;
                }
                "!warn" => {
                    config.warn = false;
                }
                "error" => {
                    config.error = true;
                }
                "!error" => {
                    config.error = false;
                }
                _ => {
                    return Err(item.to_string());
                }
            }
        }
        Ok(config)
    }
}

impl LogConfig {
    /// Enable all log output
    pub const fn enable_all() -> Self {
        Self {
            date_format: DEFAULT_DATE_FORMAT,
            color: false,
            trace: true,
            debug: true,
            info: true,
            warn: true,
            error: true,
        }
    }

    /// Disable all log output
    pub const fn disable_all() -> Self {
        Self {
            date_format: DEFAULT_DATE_FORMAT,
            color: false,
            trace: false,
            debug: false,
            info: false,
            warn: false,
            error: false,
        }
    }

    /// Create configuration from `LOG` environment variable
    pub fn from_env() -> Result<Self, LogError> {
        Self::from_env_name("LOG")
    }

    /// Create configuration from the specified environment variables
    pub fn from_env_name<S: AsRef<str>>(name: S) -> Result<Self, LogError> {
        match std::env::var(name.as_ref()) {
            Ok(s) => match Self::from_str(&s) {
                Ok(c) => Ok(c),
                Err(msg) => Err(LogError::FormatError(msg)),
            },
            Err(err) => match err {
                VarError::NotPresent => Err(LogError::NotPresent),
                VarError::NotUnicode(_) => Err(LogError::NotUnicode),
            },
        }
    }

    /// Set the display log color
    pub fn color(&mut self, enable: bool) -> &mut Self {
        self.color = enable;
        self
    }

    pub fn get_color(&self) -> bool {
        self.color
    }

    /// Set whether to enable `trace` logs
    pub fn trace(&mut self, enable: bool) -> &mut Self {
        self.trace = enable;
        self
    }

    pub fn get_trace(&self) -> bool {
        self.trace
    }

    /// Set whether to enable `debug` logs
    pub fn debug(&mut self, enable: bool) -> &mut Self {
        self.debug = enable;
        self
    }

    pub fn get_debug(&self) -> bool {
        self.debug
    }

    /// Set whether to enable `info` logs
    pub fn info(&mut self, enable: bool) -> &mut Self {
        self.info = enable;
        self
    }

    pub fn get_info(&self) -> bool {
        self.info
    }

    /// Set whether to enable `warn` logs
    pub fn warn(&mut self, enable: bool) -> &mut Self {
        self.warn = enable;
        self
    }

    pub fn get_warn(&self) -> bool {
        self.warn
    }

    /// Set whether to enable `error` logs
    pub fn error(&mut self, enable: bool) -> &mut Self {
        self.error = enable;
        self
    }

    pub fn get_error(&self) -> bool {
        self.error
    }

    /// Set the log date and time format
    ///
    /// format: [time docs](https://man7.org/linux/man-pages/man3/strftime.3.html)
    /// ```
    /// logs::LogConfig::enable_all().date_format("%c").unwrap().build();
    /// // [Fri Nov 27 13:51:59 2020] [ERROR] This is a error log
    /// ```
    pub fn date_format<S: AsRef<str>>(&mut self, format: S) -> Result<&mut Self, time::ParseError> {
        time::now().strftime(format.as_ref()).map(|_| {
            self.date_format = Box::leak(format.as_ref().to_string().into_boxed_str());
            self
        })
    }

    pub fn get_date_format(&self) -> &'static str {
        self.date_format
    }

    /// Make the configuration effective
    pub fn build(self) {
        unsafe {
            _LOG_CONFIG = self;
        }
    }
}
