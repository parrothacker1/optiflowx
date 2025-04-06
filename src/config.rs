use crate::types::config::Config;
use log;
use std::{fs::File, io::Read};

#[allow(unused_imports)]
use toml::Table;

pub fn load_config(config_path: String) -> Result<Config, std::io::Error> {
    log::debug!("Loading config file: {}", config_path);
    let config: Config;
    match File::open(config_path) {
        Ok(mut v) => {
            let mut contents = String::new();
            v.read_to_string(&mut contents)?;
            config = parse_config(contents.clone())?;
            Ok(config)
        }
        Err(v) => return Err(v),
    }
}

fn parse_config(_contents: String) -> Result<Config, std::io::Error> {
    let _config: Config;
    Ok(_config)
}
