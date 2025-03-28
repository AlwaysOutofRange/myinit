use myinit::{
    Error,
    subsystems::ipc::{
        self,
        protocol::{ResponseStatus, deserialize_response, serialize_command},
    },
};

use crate::command::Command;
use myinit::subsystems::ipc::protocol::Command as IpcCommand;

pub struct ServiceCommand;

impl Command for ServiceCommand {
    fn name(&self) -> &str {
        "service"
    }

    fn description(&self) -> &str {
        "Manage services"
    }

    fn usage(&self) -> &str {
        "service [start|stop|restart|status|list] [service_name]"
    }

    fn execute(&self, args: Vec<String>) -> Result<(), myinit::Error> {
        if args.is_empty() {
            return Err(Error {
                message: "Missing subcommand. Use 'service [start|stop|restart|status|list] [service_name]'".into(),
                fatal: false,
            });
        }

        match args[0].as_str() {
            "list" => {
                let cmd = IpcCommand::ListServices;
                let cmd_str = serialize_command(&cmd)?;
                let response_str = ipc::send_command(&cmd_str)?;
                let response = deserialize_response(&response_str)?;

                match response.status {
                    ResponseStatus::Success => {
                        if let Some(data) = response.data {
                            let services: Vec<String> =
                                serde_json::from_str(&data).unwrap_or_default();
                            println!("Available services:");
                            for service in services {
                                println!("  {}", service);
                            }
                        } else {
                            println!("No services available.");
                        }
                    }
                    ResponseStatus::Error => {
                        println!(
                            "An error occurred while listing services: {}",
                            response.message
                        );
                    }
                }
            }
            "start" => {
                if args.len() < 2 {
                    return Err(Error {
                        message: "Missing service name. Use 'service start [service_name]'".into(),
                        fatal: false,
                    });
                }

                let cmd = IpcCommand::StartService {
                    name: args[1].clone(),
                };
                let cmd_str = serialize_command(&cmd)?;
                let response_str = ipc::send_command(&cmd_str)?;
                let response = deserialize_response(&response_str)?;

                match response.status {
                    ResponseStatus::Success => {
                        println!("{}", response.message)
                    }
                    ResponseStatus::Error => {
                        println!(
                            "An error occurred while starting service {}: {}",
                            args[1], response.message
                        );
                    }
                }
            }
            "stop" => {
                if args.len() < 2 {
                    return Err(Error {
                        message: "Missing service name. Use 'service stop [service_name]'".into(),
                        fatal: false,
                    });
                }

                let cmd = IpcCommand::StopService {
                    name: args[1].clone(),
                };
                let cmd_str = serialize_command(&cmd)?;
                let response_str = ipc::send_command(&cmd_str)?;
                let response = deserialize_response(&response_str)?;

                match response.status {
                    ResponseStatus::Success => {
                        println!("{}", response.message)
                    }
                    ResponseStatus::Error => {
                        println!(
                            "An error occurred while stopping service {}: {}",
                            args[1], response.message
                        );
                    }
                }
            }
            "restart" => {
                if args.len() < 2 {
                    return Err(Error {
                        message: "Missing service name. Use 'service restart [service_name]'"
                            .into(),
                        fatal: false,
                    });
                }

                let cmd = IpcCommand::RestartService {
                    name: args[1].clone(),
                };
                let cmd_str = serialize_command(&cmd)?;
                let response_str = ipc::send_command(&cmd_str)?;
                let response = deserialize_response(&response_str)?;

                match response.status {
                    ResponseStatus::Success => {
                        println!("{}", response.message)
                    }
                    ResponseStatus::Error => {
                        println!(
                            "An error occurred while restarting service {}: {}",
                            args[1], response.message
                        );
                    }
                }
            }
            "status" => {
                let cmd = IpcCommand::GetStatus;
                let cmd_str = serialize_command(&cmd)?;
                let response_str = ipc::send_command(&cmd_str)?;
                let response = deserialize_response(&response_str)?;

                println!("System status: {}", response.message);
            }
            _ => {
                return Err(Error {
                    message: "Invalid subcommand. Use 'service [start|stop|restart|status|list] [service_name]'".into(),
                    fatal: false,
                });
            }
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(ServiceCommand)
    }
}
