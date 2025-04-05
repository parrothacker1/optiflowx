use std::collections::HashMap;

use super::peers::PeerConfig;
use super::server::ServerConfig;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: ServerConfig,                       // holds [server]
    pub peers: Option<HashMap<String, PeerConfig>>, // holds node_name -> peer based on the process
}
