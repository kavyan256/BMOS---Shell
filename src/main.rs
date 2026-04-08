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
    let parts = parse_command(command);

    if parts.is_empty() {
        return false;
    }

    let args: Vec<&str> = parts.iter().map(|s| s.as_str()).collect();

    match args[0] {
        "exit" => {
            if args.len() == 1 {
                commands::exit()
            } else {
                commands::cmd_not_fnd_err(args[0])
            }
        }
        "echo" => commands::echo(&args[1..]),
        "type" => {
            if args.len() > 1 {
                commands::type_cmd(args[1])
            } else {
                commands::type_cmd_err()
            }
        }
        "pwd" => commands::pwd(),
        "cd" => commands::cd(&args[1..]),
        cmd => commands::run_external(cmd, &args[1..]),
    }
}

fn parse_command(command: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    // Iterate through each character in the command string
    for ch in command.chars() {
        match ch {
            '\'' => in_quotes = !in_quotes,
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(ch),
        }
    }

    // Check for unclosed quotes
    if in_quotes {
        eprintln!("parse error: unclosed quote");
        return Vec::new();
    }

    //add the last part if it's not empty
    if !current.is_empty() {
        parts.push(current);
    }
    parts
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