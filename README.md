
# logs

[![Crates.io](https://img.shields.io/crates/v/logs.svg?style=flat-square)](https://crates.io/crates/logs)
[![docs.rs](https://img.shields.io/badge/docs-rs-informational.svg?style=flat-square)](https://docs.rs/logs)
![Crates.io](https://img.shields.io/crates/l/logs?style=flat-square)


> A simple terminal logger

## Usage

Add this in your `Cargo.toml`:

```toml
[dependencies]
logs = "*"
```

## Example
 
```rust
use logs::{debug, error, info, trace, warn, Logs};

fn main() {
    Logs::new().init();

    trace!("This is a trace log");
    debug!("This is a debug log");
    info!("This is a info log");
    warn!("This is a warn log");
    error!("This is a error log");
}
```

Output:

```
2022-09-06T08:38:23.490 [TRACE] This is a trace log
2022-09-06T08:38:23.490 [DEBUG] This is a debug log
2022-09-06T08:38:23.490 [INFO ] This is a info log
2022-09-06T08:38:23.490 [WARN ] This is a warn log
2022-09-06T08:38:23.490 [ERROR] This is a error log
```

## Options

```rust
use logs::{Logs, debug, error, LevelFilter};

fn main() {
    Logs::new()
        // Show log level color
        .color(true)
        // Filter log target
        .target("target")
        // Filter log level
        .level(LevelFilter::Info)
        // Filter log level from `LOG` environment variable
        .level_from_default_env()
        .unwrap()
        // Filter log level from `NAME` environment variable
        .level_from_env("NAME")
        .unwrap()
        // Filter log level from str
        .level_from_str("info")
        .unwrap()
        // Apply
        .init();
}

```
