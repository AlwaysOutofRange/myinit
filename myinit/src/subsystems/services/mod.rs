pub mod config;
pub mod supervisor;

#[derive(Debug)]
pub struct ServiceSpec {
    pub name: String,
    pub command: String,
    pub deps: Vec<String>,
}

pub struct ServiceHandle {
    pub pid: u32,
    pub spec: ServiceSpec,
}
