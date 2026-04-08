use std::io::{self, Write};

mod commands;

//prints the shell prompt
fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

//reads a line of input from the user, trims it, and returns it as a String
fn read_command() -> String {
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    command.trim().to_string()
}

//handles the command entered by the user, matches it against known commands
//returns true if the command is "exit" to signal the main loop to break, otherwise returns false
fn handle_command(command: &str) -> bool {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        return false;
    }

    match parts[0] {
        "exit" => {
            if parts.len() == 1 {
                commands::exit()
            } else {
                commands::cmd_not_fnd_err(parts[0])
            }
        }
        "echo" => commands::echo(&parts[1..]),
        "type" => {
            if parts.len() > 1 {
                commands::type_cmd(parts[1])
            } else {
                commands::type_cmd_err()
            }
        }
        cmd => commands::run_external(cmd, &parts[1..]),
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