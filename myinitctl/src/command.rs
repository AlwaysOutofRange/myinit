use myinit::Error;

pub trait Command {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn usage(&self) -> &str;
    fn execute(&self, args: Vec<String>) -> Result<(), Error>;
    fn print_help(&self) {
        println!("{} - {}", self.name(), self.description());
        println!("Usage: {}", self.usage());
    }

    fn box_clone(&self) -> Box<dyn Command>;
}

impl Clone for Box<dyn Command> {
    fn clone(&self) -> Box<dyn Command> {
        self.box_clone()
    }
}

pub struct HelpCommand {
    commands: Vec<Box<dyn Command>>,
}

impl HelpCommand {
    pub fn new(commands: Vec<Box<dyn Command>>) -> Self {
        HelpCommand { commands }
    }
}

impl Command for HelpCommand {
    fn name(&self) -> &str {
        "help"
    }

    fn description(&self) -> &str {
        "Display help information"
    }

    fn usage(&self) -> &str {
        "help [command]"
    }

    fn execute(&self, args: Vec<String>) -> Result<(), Error> {
        if args.is_empty() {
            println!("MyInitCtl - Control the MyInit init system");
            println!("Available commands:");
            for command in &self.commands {
                println!("  {} - {}", command.name(), command.description());
            }
            println!("\nType 'help [command]' for more information on a specific command.");

            return Ok(());
        }

        let cmd_name = &args[0];
        for cmd in &self.commands {
            if cmd.name() == cmd_name {
                cmd.print_help();
                return Ok(());
            }
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(HelpCommand::new(self.commands.clone()))
    }
}
