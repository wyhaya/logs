use logs::{debug, error, info, trace, warn, LevelFilter, Logs};

fn main() {
    Logs::new().level(LevelFilter::Trace).color(true).init();

    trace!("This is a trace log");
    debug!("This is a debug log");
    info!("This is a info log");
    warn!("This is a warn log");
    error!("This is a error log");
}
