//! ## Example
//!  
//! ```rust
//! use logs::{debug, error, info, trace, warn};
//!
//! fn main() {
//!     debug!("This is a debug log");
//!     trace!("This is a trace log");
//!     info!("This is a info log");
//!     warn!("This is a warn log");
//!     error!("This is a error log");
//! }
//! ```
//!
//! ## Config
//!
//! ```
//! use logs::{Config, debug, error};
//!
//! fn main() {
//!     let mut config = Config::disable_all();
//!
//!     // Disable debug! output
//!     config.debug(false);
//!
//!     // Allow error! output
//!     config.error(true);
//!
//!     // The output of `trace!` is only displayed in debug mode
//!     #[cfg(debug_assertions)]
//!     config.trace(true);
//!
//!     // Change datetime format: [Fri Nov 27 15:56:08 2020]
//!     config.date_format("%c").unwrap();
//!         
//!     config.init();
//!
//!     debug!("This is a debug log");
//!     error!("This is a error log");
//! }
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
//! use logs::Config;
//!
//! fn main() {
//!     Config::from_env().unwrap().init();
//! }
//! ```

mod config;
pub use config::{Config, DATE_FORMAT, LOG_CONFIG};
pub use time;

#[doc(hidden)]
#[macro_export]
macro_rules! _time {
    () => {
        $crate::time::now()
            .strftime(unsafe { $crate::DATE_FORMAT })
            .unwrap()
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        if unsafe { $crate::LOG_CONFIG.trace } {
            println!("[{}] \x1B[2;3m[{}]\x1B[0m {}", $crate::_time!(), "TRACE", format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! debug {
    () => {
        if unsafe { $crate::LOG_CONFIG.debug } {
            println!("[{}] \x1B[36m[{}]\x1B[0m [{}:{}]", $crate::_time!(), "DEBUG", file!(), line!());
        }
    };
    ($val:expr) => {
        if unsafe { $crate::LOG_CONFIG.debug } {
            println!("[{}] \x1B[36m[{}]\x1B[0m [{}:{}]\n{:#?}", $crate::_time!(), "DEBUG", file!(), line!(), $val);
        }
    };
    ($($arg:expr), *) => {
        if unsafe { $crate::LOG_CONFIG.debug } {
            print!("[{}] \x1B[36m[{}]\x1B[0m [{}:{}]\n{}", $crate::_time!(), "DEBUG", file!(), line!(), {
                let mut content = String::new();
                $(content += &format!("{:#?}\n", $arg);)*
                content
            });
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        if unsafe { $crate::LOG_CONFIG.info } {
            println!("[{}] \x1B[32m[{}]\x1B[0m {}", $crate::_time!(), "INFO ", format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        if unsafe { $crate::LOG_CONFIG.warn } {
            println!("[{}] \x1B[4;33m[{}]\x1B[0m {}", $crate::_time!(), "WARN ", format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        if unsafe { $crate::LOG_CONFIG.error } {
            eprintln!("[{}] \x1B[1;31m[{}]\x1B[0m {}", $crate::_time!(), "ERROR", format!($($arg)*));
        }
    };
}
