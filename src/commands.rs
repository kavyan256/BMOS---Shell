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

pub fn exit() -> bool {
    println!("Exiting ...");
    true
}

pub fn cmd_not_fnd_err(command: &str) -> bool {
    println!("{}: command not found", command);
    false
}

pub fn echo(args: &[&str]) -> bool {
    println!("{}", args.join(" "));
    false
}

pub fn type_cmd(arg: &str) -> bool {
    if BUILTINS.contains(&arg) {
        println!("{} is a shell builtin", arg);
    } else if let Some(path) = find_executable_in_path(arg) {
        println!("{} is {}", arg, path.display());
    } else {
        println!("{}: not found", arg);
    }
    false
}

pub fn run_external(cmd: &str, args: &[&str]) -> bool {
    if let Some(path) = find_executable_in_path(cmd) {
        match Command::new(&path).args(args).status() {
            Ok(_status) => false,
            Err(e) => {
                println!("Error executing {}: {}", cmd, e);
                false
            }
        }
    } else {
        println!("{}: command not found", cmd);
        false
    }
}

pub fn type_cmd_err() -> bool {
    println!("type: missing argument");
    false
}

pub fn pwd() -> bool {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("pwd: {}", e),
    }
    false
}

pub fn cd(args: &[&str]) -> bool {
    let path_str = if args.is_empty() {
        // No argument: go to home directory
        match env::var("HOME") {
            Ok(home) => home,
            Err(_) => {
                println!("cd: HOME not set");
                return false;
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
                    println!("cd: HOME not set");
                    return false;
                }
            }
        } else {
            arg.to_string()
        }
    };
    
    match env::set_current_dir(&path_str) {
        Ok(_) => false,
        Err(_) => {
            println!("cd: {}: No such file or directory", path_str);
            false
        }
    }
}