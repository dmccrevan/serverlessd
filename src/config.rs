use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub email: Option<String>,
    pub api_key: Option<String>,
    pub account_number: Option<String>,
}


pub fn build_config() -> Config {
    let mut cfg_str = String::from("");
    if std::path::Path::new("/etc/serverlessd/config").exists() {
        cfg_str = fs::read_to_string("/etc/serverlessd/config").expect("Something went wrong reading the file");
    }
    let cfg: Config = toml::from_str(cfg_str.as_ref()).unwrap();
    cfg
}