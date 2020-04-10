use daemonize::Daemonize;
use log::{error, info};

mod providers;
mod config;
mod service;

fn setup_logger() {
    let logger = fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout()); 
    logger.apply().expect("Couldn't initialize logger")
}

fn main() {
    setup_logger();

    info!("Starting up serverlessd...");

    match config::build_config() {
        Ok(cfg) => {
            let daemon = Daemonize::new();
            match daemon.start() {
                Ok(_) => info!("Successfully daemonized, de-attached from shell, running in background..."),
                Err(e) => error!("Error while daemonizing: {}", e),
            };
            match service::run_server("tcp:127.0.0.1:12345", cfg) {
                Ok(_) => info!("Successfully started varlink server, listening for connections..."),
                Err(e) => error!("Error when starting up varlink server: {}", e),
            };
        },
        Err(e) => error!("Error when building config: {:?}", e)
    }
}