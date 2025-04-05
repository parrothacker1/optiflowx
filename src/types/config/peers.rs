use serde::Deserialize;

use super::PortRange;
use super::process::ProcessConfig;

#[derive(Deserialize, Debug)]
pub struct PeerConfig {
    pub host: String,            // host of the peer so the server can connect to it
    pub accept_ports: PortRange, // the port range of the peer
    pub processes: Vec<ProcessConfig>, // the processes the peer has
}
