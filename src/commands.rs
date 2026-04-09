use std::path::PathBuf;                     //represents a filesystem path
use std::env;                               //provides functions for interacting with the environment, such as accessing environment variables
use std::fs;                                //provides functions for working with the filesystem, such as reading metadata of files
use std::os::unix::fs::PermissionsExt;      //provides Unix-specific extensions for working with file permissions, allowing us to check if a file is executable
use std::process::Command;                  //allows us to execute external programs

pub const BUILTINS: &[&str; 5] = &["exit", "echo", "type", "pwd", "cd"];

fn find_executable_in_path(cmd: &str) -> Option<PathBuf> {
    let paths = env::var("PATH").ok()?;
    for path_dir in paths.split(':') {
        let executable = PathBuf::from(path_dir).join(cmd);
        if let Ok(metadata) = fs::metadata(&executable) {           // Check if the file exists and we can read its metadata
            // Check if it's a file AND executable
            if metadata.is_file() && metadata.permissions().mode() & 0o111 != 0 {
                return Some(executable);
            }
        }
    }
    None
}

pub fn exit() -> (String, bool) {
    ("Exiting ...\n".to_string(), true)
}

pub fn cmd_not_fnd_err(command: &str) -> (String, bool) {
    (format!("{}: command not found\n", command), false)
}

pub fn echo(args: &[&str]) -> (String, bool) {
    (format!("{}\n", args.join(" ")), false)
}

pub fn type_cmd(arg: &str) -> (String, bool) {
    let output = if BUILTINS.contains(&arg) {
        format!("{} is a shell builtin\n", arg)
    } else if let Some(path) = find_executable_in_path(arg) {
        format!("{} is {}\n", arg, path.display())
    } else {
        format!("{}: not found\n", arg)
    };
    (output, false)
}

pub fn run_external(cmd: &str, args: &[&str]) -> (String, bool) {
    if let Some(path) = find_executable_in_path(cmd) {
        match Command::new(&path).args(args).output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                (stdout, false)
            }
            Err(e) => {
                (format!("Error executing {}: {}\n", cmd, e), false)
            }
        }
    } else {
        (format!("{}: command not found\n", cmd), false)
    }
}

pub fn type_cmd_err() -> (String, bool) {
    ("type: missing argument\n".to_string(), false)
}

pub fn pwd() -> (String, bool) {
    match env::current_dir() {
        Ok(path) => (format!("{}\n", path.display()), false),
        Err(e) => (format!("pwd: {}\n", e), false),
    }
}

pub fn cd(args: &[&str]) -> (String, bool) {
    let path_str = if args.is_empty() {
        // No argument: go to home directory
        match env::var("HOME") {
            Ok(home) => home,
            Err(_) => {
                return ("cd: HOME not set\n".to_string(), false);
            }
        }
    } else {
        let arg = args[0];
        // Handle ~ expansion
        if arg.starts_with("~") {
            match env::var("HOME") {
                Ok(home) => {
                    if arg == "~" {
                        home
                    } else {
                        format!("{}{}", home, &arg[1..])
                    }
                }
                Err(_) => {
                    return ("cd: HOME not set\n".to_string(), false);
                }
            }
        } else {
            arg.to_string()
        }
    };
    
    match env::set_current_dir(&path_str) {
        Ok(_) => ("".to_string(), false),
        Err(e) => {
            (format!("cd: {}: {}\n", path_str, e), false)
        }
    }
}