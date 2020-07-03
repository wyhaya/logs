use logs::{debug, error, info, trace, warn};

fn main() {
    debug!("This is a debug log");
    trace!("This is a trace log");
    info!("This is a info log");
    warn!("This is a warn log");
    error!("This is a error log");
}
