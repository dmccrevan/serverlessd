use std::sync::{Arc, RwLock};
use varlink::{Connection, OrgVarlinkServiceInterface, VarlinkService};
use varlink_derive;

use crate::service::io_serverlessd::VarlinkClientInterface;

varlink_derive::varlink_file!(
    io_serverlessd,
    "src/io.serverlessd.varlink"
);

struct ServerlessDState {
    pub status: Arc<RwLock<i32>>,
}

impl io_serverlessd::VarlinkInterface for ServerlessDState {
    fn download_worker(
        &self,
        call: &mut dyn io_serverlessd::Call_DownloadWorker,
        scriptName: String
    )-> varlink::Result<()> {
        let test = String::from("test");
        match scriptName {
            test => call.reply(io_serverlessd::Response{
                succeeded: true,
                msg: "a".into(),
                body: "a".into(),
            }),
            _ => call.reply(io_serverlessd::Response{
                succeeded: true,
                msg: "b".into(),
                body: "b".into(),
            }),
        }
    } 
}

// pub fn run

pub fn run_server() -> varlink::Result<()> {
    let status = Arc::new(RwLock::new(0));
    let stream = ServerlessDState{status};
    let my_interface = io_serverlessd::new(Box::new(stream));
    let service = VarlinkService::new(
        "org.varlink",
        "test service",
        "0.1",
        "http://varlink.org",
        vec![Box::new(my_interface)],
    );

    varlink::listen(
        service,
        "unix:io.serverlessd",
        1,
        1,
        100,
    )?;
    Ok(())
}