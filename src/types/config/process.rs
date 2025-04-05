use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProcessConfig {
    pub uuid: String,              // uuid of a process for the server to go through
    pub command: String,           // the command to execute for spinning up a new instance
    pub args: Option<Vec<String>>, // args for the command
    pub max_instances: u32,        // max no.of instances that can be spinned up
}
