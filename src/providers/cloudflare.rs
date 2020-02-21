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