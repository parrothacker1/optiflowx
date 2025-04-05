use serde::Deserialize;

pub mod config;
pub mod peers;
pub mod process;
pub mod server;

#[derive(Deserialize, Debug)]
pub struct PortRange {
    pub start: u16, // start of the port range
    pub end: u16,   // end of the port range
}

#[derive(Deserialize, Debug)]
pub struct PortMapping {
    pub port: u16,              // the port that should be mapped to a process
    pub process: ProcessConfig, // the process that a port is mapped to
}

pub use config::Config;
pub use peers::PeerConfig;
pub use process::ProcessConfig;
pub use server::ServerConfig;
