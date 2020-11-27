
# logs

[![Crates.io](https://img.shields.io/crates/v/logs.svg?style=flat-square)](https://crates.io/crates/logs)
[![docs.rs](https://img.shields.io/badge/docs-rs-informational.svg?style=flat-square)](https://docs.rs/logs)
![Crates.io](https://img.shields.io/crates/l/logs?style=flat-square)


> A simple terminal logger

## Use

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

## Features

By default, these logs will not be displayed in `--release` mode, if you need to open please add the logs features.

```toml
[dependencies]
logs = { version = "*", features = ["warn", "error" ...] }
```

```
...
[2020-07-06 15:56:08] [WARN ] This is a warn log
[2020-07-06 15:56:08] [ERROR] This is a error log
```

Change datetime format

```rust
logs::date_format("%c");
```

```
[Fri Nov 27 15:56:08 2020] [ERROR] This is a error log
```

---

