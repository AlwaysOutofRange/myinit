use crate::{Error, subsystems::services::ServiceSpec};
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
struct RawService {
    name: String,
    command: String,
    #[serde(default)]
    requires: Vec<String>,
}

pub fn load_services<P: AsRef<Path>>(path: P) -> Result<Vec<ServiceSpec>, Error> {
    let content = fs::read_to_string(path)?;
    let raw_services: Vec<RawService> = toml::from_str(&content)?;

    Ok(raw_services
        .into_iter()
        .map(|s| ServiceSpec {
            name: s.name,
            command: s.command,
            deps: s.requires,
        })
        .collect())
}
