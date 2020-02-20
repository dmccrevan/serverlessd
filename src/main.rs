use daemonize::Daemonize;
use log::{error, info};

fn setup_logger() {
    let logger = fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout()); 
    logger.apply().expect("Couldn't initialize logger")
}

fn main() {
    setup_logger();

    info!("Starting up serverlessd...");

    let daemon = Daemonize::new();
    match daemon.start() {
        Ok(_) => info!("Success!"),
        Err(e) => error!("Error {}", e),
    };
}