use serde::{Deserialize, Serialize};

pub mod config;
pub mod supervisor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSpec {
    pub name: String,
    pub command: String,
    pub deps: Vec<String>,
}

pub struct ServiceHandle {
    pub pid: u32,
    pub spec: ServiceSpec,
}
