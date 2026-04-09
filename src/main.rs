use std::io::{self, Write};
use std::fs::File;

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

    // Check for output redirection (> or 1>)
    let mut redirect_path = None;
    let mut filtered_args = args.clone();

    for i in (0..filtered_args.len()).rev() {
        if filtered_args[i] == ">" || filtered_args[i] == "1>" {
            if i + 1 < filtered_args.len() {
                redirect_path = Some(filtered_args[i + 1].to_string());
                filtered_args.truncate(i); // Remove > and path from args
            }
            break;
        }
    }

    let (output, should_exit) = match filtered_args[0] {
        "exit" => {
            if filtered_args.len() == 1 {
                commands::exit()
            } else {
                commands::cmd_not_fnd_err(filtered_args[0])
            }
        }
        "echo" => commands::echo(&filtered_args[1..]),
        "type" => {
            if filtered_args.len() > 1 {
                commands::type_cmd(filtered_args[1])
            } else {
                commands::type_cmd_err()
            }
        }
        "pwd" => commands::pwd(),
        "cd" => commands::cd(&filtered_args[1..]),
        cmd => commands::run_external(cmd, &filtered_args[1..]),
    };

    // Handle output redirection or print to stdout
    if let Some(path) = redirect_path {
        if let Ok(mut file) = File::create(&path) {
            let _ = file.write_all(output.as_bytes());
        } else {
            eprintln!("Error: cannot write to {}", path);
        }
    } else if !output.is_empty() {
        print!("{}", output);
    }

    should_exit
}

fn parse_command(command: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = command.chars().peekable();

    // Iterate through each character in the command string
    while let Some(ch) = chars.next() {
        match ch {
            '\\' if !in_quotes => {
                // Escape character outside quotes - take next char literally
                if let Some(next_ch) = chars.next() {
                    current.push(next_ch);
                }
            }
            '\'' | '"' => in_quotes = !in_quotes,
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