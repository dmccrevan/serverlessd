use std::sync::{Arc, RwLock};
use varlink::{Connection, OrgVarlinkServiceInterface, VarlinkService};
use varlink_derive;
use std::{thread, time};

use crate::service::io_serverlessd::VarlinkClientInterface;

varlink_derive::varlink_file!(
    io_serverlessd,
    "src/io.serverlessd.varlink"
);

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

pub fn run_client(conn: Arc<RwLock<varlink::Connection>>) {
    let mut iface = io_serverlessd::VarlinkClient::new(conn);
    match iface.download_worker("test".to_string()).call() {
        Ok(io_serverlessd::DownloadWorker_Reply {
            resp:
                io_serverlessd::Response {
                    succeeded: true,
                    msg: ref m,
                    body: ref b,
                }
        }) => println!("Success: {:?} {:?}", m, b),
        res => panic!("Unknown result {:?}", res),
    }
}

pub fn run_server(address: &str) -> varlink::Result<()> {
    let status = Arc::new(RwLock::new(0));
    let stream = ServerlessDState{status};
    let my_interface = io_serverlessd::new(Box::new(stream));
    let service = VarlinkService::new(
        "serverlessd",
        "serverlessd",
        "0.1",
        "https://github.com/dmccrevan/serverlessd",
        vec![Box::new(my_interface)],
    );
    varlink::listen(
        service,
        address,
        1,
        1,
        100,
    )?;
    Ok(())
}



#[test]
fn client_server_test() -> Result<()> { 
    let address = "tcp:127.0.0.1:12345";
    let client_address = "tcp:127.0.0.1:12345";
    let child = thread::spawn(move || {
        run_server(&client_address);
    });
    
    thread::sleep(time::Duration::from_secs(1));
    run_client(Connection::with_address(address)?);
    Ok(())
}