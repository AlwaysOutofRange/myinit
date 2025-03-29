use crate::command::Command;
use myinit::{
    Error,
    subsystems::ipc::{
        self,
        protocol::{Command as IpcCommand, deserialize_response, serialize_command},
    },
};

pub struct ShutdownCommand;

impl Command for ShutdownCommand {
    fn name(&self) -> &str {
        "shutdown"
    }

    fn description(&self) -> &str {
        "Shutdown or reboot the system"
    }

    fn usage(&self) -> &str {
        "shutdown [--reboot | -r]"
    }

    fn execute(&self, args: Vec<String>) -> Result<(), Error> {
        let reboot = args.iter().any(|arg| arg == "--reboot" || arg == "-r");

        let action = if reboot { "reboot " } else { "shutdown " };
        println!("Requesting system {}", action);

        let cmd = IpcCommand::Shutdown { reboot };
        let cmd_str = serialize_command(&cmd)?;

        let response_str = match ipc::send_command(&cmd_str) {
            Ok(resp) => resp,
            Err(e) => {
                return Err(Error {
                    message: format!("Failed to send {} command: {}", action, e.message),
                    fatal: false,
                });
            }
        };

        let response = deserialize_response(&response_str)?;
        println!("{}", response.message);

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(ShutdownCommand)
    }
}
