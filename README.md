
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
use logs::{debug, error, info, trace, warn};

fn main() {
    debug!("This is a debug log");
    trace!("This is a trace log");
    info!("This is a info log");
    warn!("This is a warn log");
    error!("This is a error log");
}
```

## Config

```rust
use logs::{Config, debug, error};

fn main() {
    let mut config = Config::disable_all();

    // Disable debug! output
    config.debug(false);

    // Allow error! output
    config.error(true);

    // The output of `trace!` is only displayed in debug mode
    #[cfg(debug_assertions)]
    config.trace(true);

    // Change datetime format: [Fri Nov 27 15:56:08 2020]
    config.date_format("%c").unwrap();
        
    config.init();

    debug!("This is a debug log");
    error!("This is a error log");
}

```

### env

This can be configured by reading the `LOG` environment variable, which disables all output by default

```bash
# Enable all output
# Disable debug output
# ...
export LOG='all,!debug,info,!error'
```

```rust
use logs::Config;

fn main() {
    Config::from_env().unwrap().init();
}
```