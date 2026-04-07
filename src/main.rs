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
    let parts: Vec<&str> = command.split_whitespace().collect();

    match parts.as_slice() {
        [] => false,
        ["exit"] => commands::exit(),
        ["echo", ..] => commands::echo(&parts[1..]),
        ["type", arg] => commands::type_cmd(arg),
        ["type"] => commands::type_cmd_err(),
        _ => commands::cmd_not_fnd_err(parts.first().copied().unwrap_or(command))
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