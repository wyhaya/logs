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

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(any(debug_assertions, feature = "debug"))]
        {
            println!("\x1B[36m{}\x1B[0m [{}:{}]", "[DEBUG]", file!(), line!());
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        #[cfg(any(debug_assertions, feature = "trace"))]
        {
            print!("\x1B[2;3m{}\x1B[0m ", "[TRACE]");
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
       #[cfg(any(debug_assertions, feature = "info"))]
        {
            print!("\x1B[32m{}\x1B[0m ", "[INFO]");
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
       #[cfg(any(debug_assertions, feature = "warn"))]
        {
            print!("\x1B[4;33m{}\x1B[0m ", "[WARN]");
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        #[cfg(any(debug_assertions, feature = "error"))]
        {
            eprint!("\x1B[1;31m{}\x1B[0m ", "[ERROR]");
            eprintln!($($arg)*);
        }
    };
}
