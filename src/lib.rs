pub mod subsystems;

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub fatal: bool,
}

impl From<nix::Error> for Error {
    fn from(err: nix::Error) -> Self {
        Self {
            message: format!("Unix error: {}", err),
            fatal: true,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self {
            message: format!("IO error: {}", err),
            fatal: true,
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self {
            message: format!("Toml parsing error: {}", err),
            fatal: true,
        }
    }
}
