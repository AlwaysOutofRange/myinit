use std::{
    io::{Read, Write},
    sync::{Arc, Mutex},
};

use myinit::{
    Error,
    subsystems::{
        fs,
        ipc::{
            SocketServer,
            protocol::{Command, Response, deserialize_command, serialize_response},
        },
        process::{DefaultProcessManager, ProcessManager},
        services::{ServiceSpec, supervisor::Supervisor},
    },
};

fn main() -> Result<(), Error> {
    fs::mount_essential()?;

    let pm = DefaultProcessManager;
    let sv = Arc::new(Mutex::new(Supervisor::new(pm)));

    // Start debug shell
    sv.lock().unwrap().start_service(ServiceSpec {
        name: "debug-shell".into(),
        command: "/bin/sh".into(),
        deps: Vec::new(),
    })?;

    let sv_clone = sv.clone();
    let socket_server = SocketServer::new(move |mut stream| {
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer)?;

        let cmd = match deserialize_command(&buffer) {
            Ok(cmd) => cmd,
            Err(e) => {
                let response = Response::error(&format!("Invalid command: {}", e.message));
                let response_str = serialize_response(&response)?;
                stream.write_all(response_str.as_bytes())?;

                return Ok(());
            }
        };

        let response = match cmd {
            Command::GetStatus => Response::success("System running"),
            Command::StartService { name } => {
                match sv_clone.lock().unwrap().start_service_by_name(&name) {
                    Ok(_) => Response::success(&format!("Service {} started", name)),
                    Err(e) => {
                        Response::error(&format!("Failed to start service {}: {}", name, e.message))
                    }
                }
            }
            Command::StopService { name } => match sv_clone.lock().unwrap().stop_service(&name) {
                Ok(_) => Response::success(&format!("Service {} stopped", name)),
                Err(e) => {
                    Response::error(&format!("Failed to stop service {}: {}", name, e.message))
                }
            },
            Command::RestartService { name } => {
                match sv_clone.lock().unwrap().restart_service(&name) {
                    Ok(_) => Response::success(&format!("Service {} restarted", name)),
                    Err(e) => Response::error(&format!(
                        "Failed to restart service {}: {}",
                        name, e.message
                    )),
                }
            }
            Command::ListServices => {
                let services = sv_clone.lock().unwrap().list_services();
                let services_json = serde_json::to_string(&services).unwrap_or_default();
                Response::success_with_data("Services listed", services_json)
            }
        };

        let response_str = serialize_response(&response)?;
        stream.write_all(response_str.as_bytes())?;

        Ok(())
    })?;

    socket_server.start_background_thread()?;

    loop {
        sv.lock().unwrap().get_process_manager_mut().reap()?;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
