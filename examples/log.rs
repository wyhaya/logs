use logs::{debug, error, info, trace, warn, LogConfig};

fn main() {
    trace!("This is a trace log");
    debug!("This is a debug log");
    info!("This is a info log");
    warn!("This is a warn log");
    error!("This is a error log");

    LogConfig::enable_all().color(true).build();

    trace!("This is a trace log");
    debug!("This is a debug log");
    info!("This is a info log");
    warn!("This is a warn log");
    error!("This is a error log");
}
