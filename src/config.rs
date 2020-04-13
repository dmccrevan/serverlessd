use serde::Deserialize;
use std::fs;
use xdg;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub provider: String,
    pub cloudflare: Option<CloudFlareConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CloudFlareConfig {
    pub email: Option<String>,
    pub api_key: Option<String>,
    pub account_number: Option<String>,
}

pub fn build_config() -> Result<Config, &'static str> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("serverlessd").unwrap();
    let cfg_path = xdg_dirs
        .find_config_file("config")
        .expect("Config file not found");
    let cfg_str = fs::read_to_string(cfg_path).expect("Something went wrong reading the file");
    Ok(toml::from_str(cfg_str.as_ref()).unwrap())
}
