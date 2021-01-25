use logs::{debug, error, info, trace, warn, Config};

fn main() {
    Config::from_env().unwrap().init();

    trace!("This is a trace log");
    debug!("This is a debug log");
    info!("This is a info log");
    warn!("This is a warn log");
    error!("This is a error log");
}
