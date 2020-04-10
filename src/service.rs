use log::error;
use std::sync::{Arc, RwLock};
use std::{thread, time};
use varlink::{Connection, VarlinkService};
use varlink_derive;

use crate::config;
use crate::providers;
use crate::service::io_serverlessd::VarlinkClientInterface;

varlink_derive::varlink_file!(io_serverlessd, "src/io.serverlessd.varlink");

struct ServerlessDState {
    pub cfg: config::Config,
}

impl io_serverlessd::VarlinkInterface for ServerlessDState {
    fn download_worker(
        &self,
        call: &mut dyn io_serverlessd::Call_DownloadWorker,
        script_name: String,
    ) -> varlink::Result<()> {
        match providers::cloudflare::download_worker(
            self.cfg.clone().cloudflare.unwrap(),
            script_name,
        ) {
            Ok(body) => call.reply(io_serverlessd::Response {
                succeeded: true,
                msg: "Download script successfully".into(),
                body: body,
            }),
            Err(e) => call.reply(io_serverlessd::Response {
                succeeded: false,
                msg: e.to_string(),
                body: "".into(),
            }),
        }
    }
    fn upload_worker(
        &self,
        call: &mut dyn io_serverlessd::Call_UploadWorker,
        script_name: String,
        script: String,
    ) -> varlink::Result<()> {
        match providers::cloudflare::upload_worker(
            self.cfg.clone().cloudflare.unwrap(),
            script_name,
            script,
        ) {
            Ok(body) => call.reply(io_serverlessd::Response {
                succeeded: true,
                msg: "Successfully uploaded script".into(),
                body: body,
            }),
            Err(e) => call.reply(io_serverlessd::Response {
                succeeded: false,
                msg: e.to_string(),
                body: "".into(),
            }),
        }
    }
    fn delete_worker(
        &self,
        call: &mut dyn io_serverlessd::Call_DeleteWorker,
        script_name: String,
    ) -> varlink::Result<()> {
        match providers::cloudflare::delete_worker(
            self.cfg.clone().cloudflare.unwrap(),
            script_name,
        ) {
            Ok(body) => call.reply(io_serverlessd::Response {
                succeeded: true,
                msg: "Successfully deleted script".into(),
                body: body,
            }),
            Err(e) => call.reply(io_serverlessd::Response {
                succeeded: false,
                msg: e.to_string(),
                body: "".into(),
            }),
        }
    }
    fn list_workers(&self, call: &mut dyn io_serverlessd::Call_ListWorkers) -> varlink::Result<()> {
        match providers::cloudflare::list_workers(self.cfg.clone().cloudflare.unwrap()) {
            Ok(body) => call.reply(io_serverlessd::Response {
                succeeded: true,
                msg: "Successfully got list of workers".into(),
                body: body,
            }),
            Err(e) => call.reply(io_serverlessd::Response {
                succeeded: false,
                msg: e.to_string(),
                body: "".into(),
            }),
        }
    }
}

pub fn run_server(address: &str, cfg: config::Config) -> varlink::Result<()> {
    let stream = ServerlessDState { cfg };
    let my_interface = io_serverlessd::new(Box::new(stream));
    let service = VarlinkService::new(
        "serverlessd",
        "serverlessd",
        "0.1",
        "https://github.com/dmccrevan/serverlessd",
        vec![Box::new(my_interface)],
    );
    varlink::listen(service, address, 1, 1, 100)?;
    Ok(())
}

// run_client is a testing client function to run rudimentary API calls to my server
pub fn run_client(conn: Arc<RwLock<varlink::Connection>>) {
    let mut iface = io_serverlessd::VarlinkClient::new(conn);
    let script_name = String::from("serverless-d-test");
    let script = String::from(
        "addEventListener('fetch', event => { event.respondWith(fetch(event.request)) })",
    );
    match iface.upload_worker(script_name.clone(), script).call() {
        Ok(io_serverlessd::UploadWorker_Reply {
            resp:
                io_serverlessd::Response {
                    succeeded: ref s,
                    msg: ref m,
                    body: ref b,
                },
        }) => println!("Varlink Response: {} {} {}", s, m, b),
        res => error!("Unknown response: {:?}", res),
    }
    match iface.list_workers().call() {
        Ok(io_serverlessd::ListWorkers_Reply {
            resp:
                io_serverlessd::Response {
                    succeeded: true,
                    msg: ref m,
                    body: ref b,
                },
        }) => println!("Success: {:?} {:?}", m, b),
        res => error!("Unknown result {:?}", res),
    }
    match iface.download_worker(script_name.clone()).call() {
        Ok(io_serverlessd::DownloadWorker_Reply {
            resp:
                io_serverlessd::Response {
                    succeeded: true,
                    msg: ref m,
                    body: ref b,
                },
        }) => println!("Success: {:?} {:?}", m, b),
        res => error!("Unknown result {:?}", res),
    }
    match iface.delete_worker(script_name.clone()).call() {
        Ok(io_serverlessd::DeleteWorker_Reply {
            resp:
                io_serverlessd::Response {
                    succeeded: true,
                    msg: ref m,
                    body: ref b,
                },
        }) => println!("Success: {:?} {:?}", m, b),
        res => error!("Unknown result {:?}", res),
    }
    match iface.download_worker(script_name).call() {
        Ok(io_serverlessd::DownloadWorker_Reply {
            resp:
                io_serverlessd::Response {
                    succeeded: true,
                    msg: ref m,
                    body: ref b,
                },
        }) => println!("Success: {:?} {:?}", m, b),
        res => error!("Unknown result {:?}", res),
    }
}

#[test]
fn client_server_test() -> varlink::Result<()> {
    let address = "tcp:127.0.0.1:12345";
    let client_address = "tcp:127.0.0.1:12345";
    let cfg = config::build_config().unwrap();
    let child = thread::spawn(move || {
        run_server(&client_address, cfg);
    });
    thread::sleep(time::Duration::from_secs(1));
    run_client(Connection::with_address(address)?);
    Ok(())
}
