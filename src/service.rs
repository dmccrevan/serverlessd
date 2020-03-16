use std::sync::{Arc, RwLock};
use varlink::{Connection, OrgVarlinkServiceInterface, VarlinkService};
use varlink_derive;
use std::{thread, time};

use crate::providers;
use crate::config;
use crate::service::io_serverlessd::VarlinkClientInterface;


varlink_derive::varlink_file!(
    io_serverlessd,
    "src/io.serverlessd.varlink"
);

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct ServerlessDState {
    pub cfg: config::Config,
    //pub cfg: Arc<RwLock<config::Config>>,
}

impl io_serverlessd::VarlinkInterface for ServerlessDState {
    fn download_worker(
        &self,
        call: &mut dyn io_serverlessd::Call_DownloadWorker,
        script_name: String
    )-> varlink::Result<()> {
        let cfg = self.cfg.clone();
        match providers::cloudflare::download_worker(cfg.cloudflare.unwrap(), script_name) {
            Ok(body) => call.reply(io_serverlessd::Response{
                succeeded: true,
                msg: "Download script successfully".into(), 
                body: body
            }),
            Err(e) => call.reply(io_serverlessd::Response{
                succeeded: false,
                msg: "Failed to download script".into(),
                body: "".into()
            })
        }
    } 
}

pub fn run_client(conn: Arc<RwLock<varlink::Connection>>) {
    let mut iface = io_serverlessd::VarlinkClient::new(conn);
    match iface.download_worker("dan".to_string()).call() {
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
    let cfg = config::build_config();
    let stream = ServerlessDState{cfg};
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