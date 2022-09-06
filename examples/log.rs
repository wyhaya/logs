use logs::{debug, error, info, trace, warn, Level, Logs};

fn main() {
    Logs::new().level(Level::Trace).color(true).init();

    trace!("This is a trace log");
    debug!("This is a debug log");
    info!("This is a info log");
    warn!("This is a warn log");
    error!("This is a error log");
}
