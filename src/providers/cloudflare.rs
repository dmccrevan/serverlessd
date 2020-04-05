use crate::config;


#[tokio::main]
pub async fn download_worker(cfg: config::CloudFlareConfig, script_name: String) -> Result<String, reqwest::Error> {
    let url = format!("https://api.cloudflare.com/client/v4/accounts/{}/workers/scripts/{}", cfg.account_number.unwrap(), script_name);
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("X-Auth-Email", cfg.email.unwrap())
        .header("X-Auth-Key", cfg.api_key.unwrap())
        .send()
        .await?;
    let body = resp.text().await?;
    Ok(body)
}


#[tokio::main]
pub async fn upload_worker(cfg: config::CloudFlareConfig, script_name: String, script: String) -> Result<String, reqwest::Error> {
    let url = format!("https://api.cloudflare.com/client/v4/accounts/{}/workers/scripts/{}", cfg.account_number.unwrap(), script_name);
    let client = reqwest::Client::new();
    let resp = client
        .put(&url)
        .body(script)
        .header("X-Auth-Email", cfg.email.unwrap())
        .header("X-Auth-Key", cfg.api_key.unwrap())
        .header("Content-Type", "application/javascript")
        .send()
        .await?;
    let body = resp.text().await?;
    Ok(body)
}


#[test]
fn upload_worker_test() {
    let cfg = config::build_config();
    let script = String::from("addEventListener('fetch', event => { event.respondWith(fetch(event.request)) })");
    let script_name = String::from("uploadtest");
    match upload_worker(cfg.cloudflare.unwrap(), script_name, script) {
        Ok(body) => println!("body {}", body),
        Err(e) => println!("error {}", e),
    };
}