use logs::{debug, error, info, trace, warn, LogConfig};

fn main() {
    LogConfig::from_env().unwrap_or_default().init();

    trace!("This is a trace log");
    debug!("This is a debug log");
    info!("This is a info log");
    warn!("This is a warn log");
    error!("This is a error log");
}
