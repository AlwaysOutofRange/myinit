pub mod protocol;

use std::{
    io::{Read, Write},
    net::Shutdown,
    os::unix::{
        fs::PermissionsExt,
        net::{UnixListener, UnixStream},
    },
    path::Path,
    thread,
};

use crate::Error;

pub const SOCKET_PATH: &str = "/run/myinit.sock";

pub struct SocketServer {
    listener: UnixListener,
    handler: Box<dyn Fn(UnixStream) -> Result<(), Error> + Send + 'static>,
}

impl SocketServer {
    pub fn new<F>(handler: F) -> Result<Self, Error>
    where
        F: Fn(UnixStream) -> Result<(), Error> + Send + 'static,
    {
        if Path::new(SOCKET_PATH).exists() {
            std::fs::remove_file(SOCKET_PATH).map_err(|e| Error {
                message: format!("Failed to remove existing socket: {}", e),
                fatal: true,
            })?;
        }

        let listener = UnixListener::bind(SOCKET_PATH).map_err(|e| Error {
            message: format!("Failed to bind socket: {}", e),
            fatal: true,
        })?;

        if let Err(e) = std::fs::set_permissions(
            Path::new(SOCKET_PATH),
            std::fs::Permissions::from_mode(0o666),
        ) {
            eprintln!("Failed to set permissions on socket: {}", e);
        }

        Ok(Self {
            listener,
            handler: Box::new(handler),
        })
    }

    pub fn start_background_thread(self) -> Result<(), Error> {
        thread::spawn(move || {
            for stream in self.listener.incoming() {
                match stream {
                    Ok(stream) => {
                        if let Err(e) = (self.handler)(stream) {
                            eprintln!("Error handling socket connection: {}", e.message)
                        }
                    }
                    Err(e) => {
                        eprintln!("Error accepting socket connection: {}", e.to_string())
                    }
                }
            }
        });

        Ok(())
    }
}

pub fn send_command(command: &str) -> Result<String, Error> {
    let mut stream = UnixStream::connect(SOCKET_PATH).map_err(|e| Error {
        message: format!("Failed to connect to myinit socket: {}", e),
        fatal: false,
    })?;

    stream.write_all(command.as_bytes()).map_err(|e| Error {
        message: format!("Failed to send command: {}", e),
        fatal: false,
    })?;
    stream.shutdown(Shutdown::Write)?;

    let mut response = String::new();
    stream.read_to_string(&mut response).map_err(|e| Error {
        message: format!("Failed to read response: {}", e),
        fatal: false,
    })?;

    Ok(response)
}
