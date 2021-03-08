//! ## Example
//!  
//! ```rust
//! use logs::{debug, error, info, trace, warn};
//!
//! debug!("This is a debug log");
//! trace!("This is a trace log");
//! info!("This is a info log");
//! warn!("This is a warn log");
//! error!("This is a error log");
//! ```
//!
//! ## Config
//!
//! ```
//! use logs::{LogConfig, debug, error};
//!
//! let mut config = LogConfig::disable_all();
//!
//! // Disable debug! output
//! config.debug(false);
//!
//! // Allow error! output
//! config.error(true);
//!
//! // The output of `trace!` is only displayed in debug mode
//! #[cfg(debug_assertions)]
//! config.trace(true);
//!
//! // Change datetime format: [Fri Nov 27 15:56:08 2020]
//! config.date_format("%c").unwrap();
//!      
//! config.build();
//!
//! debug!("This is a debug log");
//! error!("This is a error log");
//!
//! ```
//!
//! ### env
//!
//! This can be configured by reading the `LOG` environment variable, which disables all output by default
//!
//! ```bash
//! # Enable all output
//! # Disable debug output
//! # ...
//! export LOG='all,!debug,info,!error'
//! ```
//!
//! ```
//! use logs::LogConfig;
//!
//! LogConfig::from_env().unwrap_or_default().build();
//! ```

mod config;
pub use config::{LogConfig, LogError, _LOG_CONFIG};
#[doc(hidden)]
pub use time;

#[doc(hidden)]
#[macro_export]
macro_rules! _time {
    () => {
        $crate::time::now()
            .strftime(unsafe { $crate::_LOG_CONFIG.get_date_format() })
            .unwrap()
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        if unsafe { $crate::_LOG_CONFIG.get_trace() } {
            if unsafe { $crate::_LOG_CONFIG.get_color() } {
                println!("[{}] \x1B[2;3m[TRACE]\x1B[0m {}", $crate::_time!(), format!($($arg)*));
            } else {
                println!("[{}] [TRACE] {}", $crate::_time!(), format!($($arg)*));
            }
        }
    };
}

#[macro_export]
macro_rules! debug {
    () => {
        if unsafe { $crate::_LOG_CONFIG.get_debug() } {
            if unsafe { $crate::_LOG_CONFIG.get_color() } {
                println!("[{}] \x1B[36m[DEBUG]\x1B[0m [{}:{}]", $crate::_time!(), file!(), line!());
            } else {
                println!("[{}] [DEBUG] [{}:{}]", $crate::_time!(), file!(), line!());
            }
        }
    };
    ($val:expr) => {
        if unsafe { $crate::_LOG_CONFIG.get_debug() } {
            if unsafe { $crate::_LOG_CONFIG.get_color() } {
                println!("[{}] \x1B[36m[DEBUG]\x1B[0m [{}:{}]\n{:#?}", $crate::_time!(), file!(), line!(), $val);
            } else {
                println!("[{}] [DEBUG] [{}:{}]\n{:#?}", $crate::_time!(), file!(), line!(), $val);
            }
        }
    };
    ($($arg:expr), *) => {
        if unsafe { $crate::_LOG_CONFIG.get_debug() } {
            if unsafe { $crate::_LOG_CONFIG.get_color() } {
                print!("[{}] \x1B[36m[DEBUG]\x1B[0m [{}:{}]\n{}", $crate::_time!(), file!(), line!(), {
                    let mut content = String::new();
                    $(content += &format!("{:#?}\n", $arg);)*
                    content
                });
            } else {
                print!("[{}] [DEBUG] [{}:{}]\n{}", $crate::_time!(), file!(), line!(), {
                    let mut content = String::new();
                    $(content += &format!("{:#?}\n", $arg);)*
                    content
                });
            }
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        if unsafe { $crate::_LOG_CONFIG.get_info() } {
            if unsafe { $crate::_LOG_CONFIG.get_color() } {
                println!("[{}] \x1B[32m[INFO ]\x1B[0m {}", $crate::_time!(), format!($($arg)*));
            } else {
                println!("[{}] [INFO ] {}", $crate::_time!(), format!($($arg)*));
            }
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        if unsafe { $crate::_LOG_CONFIG.get_warn() } {
            if unsafe { $crate::_LOG_CONFIG.get_color() } {
                println!("[{}] \x1B[4;33m[WARN ]\x1B[0m {}", $crate::_time!(), format!($($arg)*));
            } else {
                println!("[{}] [WARN ] {}", $crate::_time!(), format!($($arg)*));
            }
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        if unsafe { $crate::_LOG_CONFIG.get_error() } {
            if unsafe { $crate::_LOG_CONFIG.get_color() } {
                eprintln!("[{}] \x1B[1;31m[ERROR]\x1B[0m {}", $crate::_time!(), format!($($arg)*));
            } else {
                eprintln!("[{}] [ERROR] {}", $crate::_time!(), format!($($arg)*));
            }
        }
    };
}
