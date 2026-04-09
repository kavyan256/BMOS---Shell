use std::io::{self, Write};
use std::fs::{File, OpenOptions};

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

    // Check for output redirection (> or 1> for stdout, 2> for stderr, >> for append)
    let mut stdout_redirect = None;
    let mut stderr_redirect = None;
    let mut stdout_append = false;
    let mut filtered_args = args.clone();

    for i in (0..filtered_args.len()).rev() {
        if filtered_args[i] == ">>" {
            if i + 1 < filtered_args.len() {
                stdout_redirect = Some(filtered_args[i + 1].to_string());
                stdout_append = true;
                filtered_args.truncate(i);
            }
            break;
        } else if filtered_args[i] == ">" || filtered_args[i] == "1>" {
            if i + 1 < filtered_args.len() {
                stdout_redirect = Some(filtered_args[i + 1].to_string());
                stdout_append = false;
                filtered_args.truncate(i);
            }
            break;
        }
    }

    for i in (0..filtered_args.len()).rev() {
        if filtered_args[i] == "2>" {
            if i + 1 < filtered_args.len() {
                stderr_redirect = Some(filtered_args[i + 1].to_string());
                filtered_args.truncate(i);
            }
            break;
        }
    }

    let (output, errors, should_exit) = match filtered_args[0] {
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

    // Handle stdout redirection
    if let Some(path) = stdout_redirect {
        let result = if stdout_append {
            // Append mode: open existing file or create if it doesn't exist
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
        } else {
            // Truncate mode: create or overwrite
            File::create(&path)
        };

        if let Ok(mut file) = result {
            let _ = file.write_all(output.as_bytes());
        } else {
            eprintln!("Error: cannot write to {}", path);
        }
    } else if !output.is_empty() {
        print!("{}", output);
    }

    // Handle stderr redirection
    if let Some(path) = stderr_redirect {
        if let Ok(mut file) = File::create(&path) {
            let _ = file.write_all(errors.as_bytes());
        } else {
            eprintln!("Error: cannot write to {}", path);
        }
    } else if !errors.is_empty() {
        eprint!("{}", errors);
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