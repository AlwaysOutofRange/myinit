use crate::command::HelpCommand;

use super::command::Command;

pub fn get_commands() -> Vec<Box<dyn Command>> {
    let mut commands: Vec<Box<dyn Command>> = vec![];

    let help_command = Box::new(HelpCommand::new(commands.clone()));

    commands.push(help_command);

    commands
}
