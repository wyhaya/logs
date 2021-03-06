use std::env::VarError;
use std::str::FromStr;

// Default date format: 0000-00-00 00:00:00
#[doc(hidden)]
pub static mut DATE_FORMAT: &str = "%F %T";
#[doc(hidden)]
pub static mut LOG_CONFIG: LogConfig = LogConfig::enable_all();

#[derive(Debug, Clone)]
pub enum LogError {
    NotPresent,
    NotUnicode,
    FormatError(String),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LogConfig {
    pub trace: bool,
    pub debug: bool,
    pub info: bool,
    pub warn: bool,
    pub error: bool,
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
                    return Err(format!("Invalid ENV parameter: {}", item));
                }
            }
        }
        Ok(config)
    }
}

impl LogConfig {
    pub const fn enable_all() -> Self {
        Self {
            trace: true,
            debug: true,
            info: true,
            warn: true,
            error: true,
        }
    }

    pub const fn disable_all() -> Self {
        Self {
            trace: false,
            debug: false,
            info: false,
            warn: false,
            error: false,
        }
    }

    pub fn from_env() -> Result<Self, LogError> {
        match std::env::var("LOG") {
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

    pub fn trace(&mut self, enable: bool) -> &mut Self {
        self.trace = enable;
        self
    }

    pub fn debug(&mut self, enable: bool) -> &mut Self {
        self.debug = enable;
        self
    }

    pub fn info(&mut self, enable: bool) -> &mut Self {
        self.info = enable;
        self
    }

    pub fn warn(&mut self, enable: bool) -> &mut Self {
        self.warn = enable;
        self
    }

    pub fn error(&mut self, enable: bool) -> &mut Self {
        self.error = enable;
        self
    }

    // TODO
    /// Change date and time format
    ///
    /// [time docs](https://docs.rs/time/0.2.23/time/?search=#formatting)
    /// ```no-run
    /// config.date_format("%c").unwrap();
    /// [Fri Nov 27 13:51:59 2020] [ERROR] This is a error log
    /// ```
    pub fn date_format<S: AsRef<str>>(&mut self, format: S) -> Result<&mut Self, ()> {
        if time::now().strftime(format.as_ref()).is_ok() {
            unsafe {
                DATE_FORMAT = Box::leak(format.as_ref().to_string().into_boxed_str());
            }
            Ok(self)
        } else {
            Err(())
        }
    }

    /// Make the configuration effective
    pub fn init(self) {
        unsafe {
            LOG_CONFIG = self;
        }
    }
}
