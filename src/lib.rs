//!  ```
//!  use logs::{debug, error, info, trace, warn};
//!  
//!  fn main() {
//!      debug!("This is a debug log");
//!      trace!("This is a trace log");
//!      info!("This is a info log");
//!      warn!("This is a warn log");
//!      error!("This is a error log");
//!  }
//!  ```
//! ## Features
//!
//! By default, these logs will not be displayed in `--release` mode, if you need to open please add the logs features.
//!
//! ```no-run
//! [dependencies]
//! logs = { version = "*", features = ["warn", "error" ...] }
//! ```
//! ```no-run
//! ...
//! [2020-07-06 15:56:08] [WARN ] This is a warn log
//! [2020-07-06 15:56:08] [ERROR] This is a error log
//! ```
//! Change datetime format
//! ```no-run
//! logs::date_format("%c");
//! ```
//! ```no-run
//! [Fri Nov 27 15:56:08 2020] [ERROR] This is a error log
//! ```

use cfg_if::cfg_if;
pub use time;

// Default date format: 0000-00-00 00:00:00
#[doc(hidden)]
pub static mut DATE_FORMAT: &str = "%F %T";

/// Change date and time format
///
/// [time docs](https://docs.rs/time/0.2.23/time/?search=#formatting)
/// ```no-run
/// date_format("%c");
/// [Fri Nov 27 13:51:59 2020] [ERROR] This is a error log
/// ```
pub fn date_format(s: &str) -> bool {
    time::now()
        .strftime(&s)
        .map(|_| unsafe {
            DATE_FORMAT = Box::leak(s.to_string().into_boxed_str());
        })
        .is_ok()
}

#[doc(hidden)]
#[macro_export]
macro_rules! _time {
    () => {
        $crate::time::now()
            .strftime(unsafe { $crate::DATE_FORMAT })
            .unwrap()
    };
}

cfg_if! {
    if #[cfg(any(debug_assertions, feature = "debug"))] {
        #[macro_export]
        macro_rules! debug {
            () => {
                println!("[{}] \x1B[36m[{}]\x1B[0m [{}:{}]", $crate::_time!(), "DEBUG", file!(), line!());
            };
            ($val:expr) => {
                println!("[{}] \x1B[36m[{}]\x1B[0m [{}:{}]\n{:#?}", $crate::_time!(), "DEBUG", file!(), line!(), $val);
            };
            ($($arg:expr), *) => {
                print!("[{}] \x1B[36m[{}]\x1B[0m [{}:{}]\n{}", $crate::_time!(), "DEBUG", file!(), line!(), {
                    let mut content = String::new();
                    $(content += &format!("{:#?}\n", $arg);)*
                    content
                });
            };
        }
    } else {
        #[macro_export]
        macro_rules! debug {
            ($($arg:tt)*) => { };
        }
    }
}

cfg_if! {
    if #[cfg(any(debug_assertions, feature = "trace"))] {
        #[macro_export]
        macro_rules! trace {
            ($($arg:tt)*) => {
                println!("[{}] \x1B[2;3m[{}]\x1B[0m {}", $crate::_time!(), "TRACE", format!($($arg)*));
            };
        }
    }else {
        #[macro_export]
        macro_rules! trace {
            ($($arg:tt)*) => { };
        }
    }
}

cfg_if! {
    if #[cfg(any(debug_assertions, feature = "info"))] {
        #[macro_export]
        macro_rules! info {
            ($($arg:tt)*) => {
                println!("[{}] \x1B[32m[{}]\x1B[0m {}", $crate::_time!(), "INFO ", format!($($arg)*));
            };
        }
    }else {
        #[macro_export]
        macro_rules! info {
            ($($arg:tt)*) => { };
        }
    }
}

cfg_if! {
    if #[cfg(any(debug_assertions, feature = "warn"))] {
        #[macro_export]
        macro_rules! warn {
            ($($arg:tt)*) => {
                println!("[{}] \x1B[4;33m[{}]\x1B[0m {}", $crate::_time!(), "WARN ", format!($($arg)*));
            };
        }
    }else {
        #[macro_export]
        macro_rules! warn {
            ($($arg:tt)*) => { };
        }
    }
}

cfg_if! {
    if #[cfg(any(debug_assertions, feature = "error"))] {
        #[macro_export]
        macro_rules! error {
            ($($arg:tt)*) => {
                eprintln!("[{}] \x1B[1;31m[{}]\x1B[0m {}", $crate::_time!(), "ERROR", format!($($arg)*));
            };
        }
    }else {
        #[macro_export]
        macro_rules! error {
            ($($arg:tt)*) => { };
        }
    }
}
