use std::io::{self, Write};

mod commands;

fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

fn read_command() -> String {
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    command.trim().to_string()
}

fn handle_command(command: &str) -> bool {
    match command {
        "exit" => commands::exit(),
        _ => commands::error_cmd_not_fnd(command)
    }
}

fn main() {
    loop {
        print_prompt();
        let command = read_command();
        if handle_command(&command) {
            break;
        }
    }
}