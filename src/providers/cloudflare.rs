use crate::config;
use log::{debug};


#[tokio::main]
pub async fn download_worker(cfg: config::CloudFlareConfig, script_name: String) -> Result<String, reqwest::Error> {
    let url = format!("https://api.cloudflare.com/client/v4/accounts/{}/workers/scripts/{}", cfg.account_number.unwrap(), script_name);
    let client = reqwest::Client::new();
    debug!("Sending GET request to download worker script from cloudflare");
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
    debug!("Sending PUT request to upload worker script from cloudflare");
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


#[tokio::main]
pub async fn delete_worker(cfg: config::CloudFlareConfig, script_name: String) -> Result<String, reqwest::Error> {
    let url = format!("https://api.cloudflare.com/client/v4/accounts/{}/workers/scripts/{}", cfg.account_number.unwrap(), script_name);
    let client = reqwest::Client::new();
    debug!("Sending DELETE request to delete worker script from cloudflare");
    let resp = client
        .delete(&url)
        .header("X-Auth-Email", cfg.email.unwrap())
        .header("X-Auth-Key", cfg.api_key.unwrap())
        .send()
        .await?;
    let body = resp.text().await?;
    Ok(body)
}