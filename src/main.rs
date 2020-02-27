use daemonize::Daemonize;
use log::{error, info};
use std::fs::File;
use std::io::Write;

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
    let cfg = config::build_config();
    setup_logger();

    info!("Starting up serverlessd...");

    service::run_server("tcp:127.0.0.1:12345");

    /*
    let daemon = Daemonize::new();

    match daemon.start() {
        Ok(_) => info!("Successfully daemonized, de-attached from shell, running in background..."),
        Err(e) => error!("Error while daemonizing: {}", e),
    };
    
    
    match providers::cloudflare::download_worker(cfg.cloudflare.unwrap(), String::from("dan")) {
        Ok(body) => {
            info!("Body: {}\n", body);
            let mut f = File::create("/tmp/tmp.txt").expect("Unable to create file");
            f.write_all(body.as_bytes()).expect("Unable to write data");
       },
        Err(e) => error!("Err {}", e),
    };*/
}