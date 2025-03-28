use std::{env, process::exit};

use commands::get_commands;
use myinit::Error;

mod command;
mod commands;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e.message);
        exit(if e.fatal { 1 } else { 0 });
    }
}

fn run() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let cmd_name = if args.len() > 1 { &args[1] } else { "" };
    let commands = get_commands();

    for cmd in &commands {
        if cmd.name() == cmd_name {
            if args.len() > 1 {
                return cmd.execute(args[2..].to_vec());
            }
        }
    }

    if !cmd_name.is_empty() {
        println!("Unknown command: {}", cmd_name);
    }

    println!("Available commands:");
    for cmd in &commands {
        println!("  {} - {}", cmd.name(), cmd.description());
    }

    Ok(())
}
