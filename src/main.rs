mod config;
mod types;

use env_logger;
use log;
use std::process;

fn main() {
    env_logger::init();
    let config_file: &str = match cfg!(debug_assertions) {
        true => "./config/leader_node.toml",
        false => "/etc/config.toml",
    };
    log::debug!("Using config: {}", config_file);
    match config::load_config(config_file.to_string()) {
        Ok(_) => {
            print!("test");
        }
        Err(e) => {
            log::error!("Failed to load {}: {}", config_file, e.to_string());
            process::exit(1);
        }
    }
    print!("Hello WOrld");
    return;
}
