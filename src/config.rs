use serde::Deserialize;
use std::{env, fs, path};

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

fn get_global_config_file() -> Result<path::PathBuf, &'static str> {
    let cfg_path = path::PathBuf::from("/etc/serverlessd/config");
    if cfg_path.exists() {
        Ok(cfg_path)
    } else {
        Err("Config file does not exist, config file must be located at ~/.config/serverless/config or /etc/serverlessd/config")
    }
}

fn get_config_file() -> Result<path::PathBuf, &'static str> {
    match env::home_dir() {
        Some(p) => {
            let mut cfg_path = p.join(".config/serverlessd/config");
            if cfg_path.exists() {
                Ok(cfg_path)
            } else {
                get_global_config_file()
            }
        }
        None => get_global_config_file(),
    }
}

pub fn build_config() -> Result<Config, &'static str> {
    match get_config_file() {
        Ok(p) => {
            let cfg_str = fs::read_to_string(p).expect("Something went wrong reading the file");
            Ok(toml::from_str(cfg_str.as_ref()).unwrap())
        }
        Err(e) => Err(e),
    }
}

#[test]
fn get_config_file_test() {
    match get_config_file() {
        Ok(p) => println!("Found config: {:?}", p),
        Err(e) => println!("Error: {:?}", e),
    };
}
