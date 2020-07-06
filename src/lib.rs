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
//!
//! By default, the log does not include the recording time, to display please add logs features.
//!
//! ```no-run
//! [dependencies]
//! logs = { version = "*", features = ["time"] }
//! ```
//!
//! ```no-run
//! ...
//! [2020-07-06 15:56:08] [WARN ] This is a warn log
//! [2020-07-06 15:56:08] [ERROR] This is a error log
//! ```

#[cfg(feature = "time")]
pub use time;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "time")] {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! _time {
            ($print:expr) => {
                if $print {
                    print!("[{}] ", $crate::time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap());
                }else {
                    eprint!("[{}] ", $crate::time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap());
                }
            };
        }
    } else {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! _time {
            ($_:expr) => {};
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! _print {
    ($($arg:tt)*) => {
        $crate::_time!(true);
        print!($($arg)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _println {
    ($($arg:tt)*) => {
        $crate::_time!(true);
        println!($($arg)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _eprint {
    ($($arg:tt)*) => {
        $crate::_time!(false);
        eprint!($($arg)*);
    };
}

cfg_if! {
    if #[cfg(any(debug_assertions, feature = "debug"))] {
        #[macro_export]
        macro_rules! debug {
            () => {
                $crate::_println!("\x1B[36m{}\x1B[0m [{}:{}]", "[DEBUG]", file!(), line!());
            };
            ($val:expr) => {
                $crate::_println!("\x1B[36m{}\x1B[0m [{}:{}]", "[DEBUG]", file!(), line!());
                println!("{:#?}", $val);
            };
            ($($arg:tt)*) => {
                $crate::_println!("\x1B[36m{}\x1B[0m [{}:{}]", "[DEBUG]", file!(), line!());
                println!($($arg)*);
             };
        }
    } else {
        #[macro_export]
        macro_rules! debug {
            ($($arg:tt)*) => {};
        }
    }
}

cfg_if! {
    if #[cfg(any(debug_assertions, feature = "trace"))] {
        #[macro_export]
        macro_rules! trace {
            ($($arg:tt)*) => {
                $crate::_print!("\x1B[2;3m{}\x1B[0m ", "[TRACE]");
                println!($($arg)*);
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
                $crate::_print!("\x1B[32m{}\x1B[0m ", "[INFO ]");
                println!($($arg)*);
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
                $crate::_print!("\x1B[4;33m{}\x1B[0m ", "[WARN ]");
                println!($($arg)*);
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
                $crate::_eprint!("\x1B[1;31m{}\x1B[0m ", "[ERROR]");
                eprintln!($($arg)*);
            };
        }
    }else {
        #[macro_export]
        macro_rules! error {
            ($($arg:tt)*) => { };
        }
    }
}
