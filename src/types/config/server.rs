use super::process::ProcessConfig;
use super::{PortMapping, PortRange};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub hostname: String,              // holds hostname of the server
    pub listen_ports: PortRange, // holds the port range from start to end (start servers on these ports)
    pub max_connections: u32,    // Max concurrent TCP connections at a time
    pub cleanup_interval: u64,   // Time in seconds to kill a process running on the server
    pub mapping: Vec<PortMapping>, // mapping each port to a process
    pub processes: Vec<ProcessConfig>, // list of processes that this server will manage
}
