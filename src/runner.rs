use crate::builtin_command::BuiltinCommand;
use crate::error::not_found::NotFound;
use crate::output_config::OutputConfig;
use crate::path_finder::PathFinder;
use std::env;
use std::io::Write;
use std::ops::ControlFlow;
use std::path::Path;

//runner file is responsible for actually executing commands, whether they are built-in commands or executables. 
//It defines functions for each built-in command and a function to execute external commands. Each function takes 
//the necessary arguments and an OutputConfig to handle output redirection, and returns a ControlFlow to indicate 
//whether to continue executing further commands or to break. The runner serves as the core execution engine of the 
//shell, coordinating the execution of commands based on their type and handling their output appropriately.

// command list
// 1. exit
// 2. echo
// 3. type
// 4. pwd
// 5. cd
// 6. executable

pub fn exit() -> ControlFlow<()> {
    ControlFlow::Break(())
}

pub fn echo(args: &[String], mut output_config: OutputConfig) -> ControlFlow<()> {
    writeln!(output_config.stdout, "{}", args.join(" ")).unwrap();
    ControlFlow::Continue(())
}

//r# is a raw identifier to tell rust to treat type as a regular identifier and not a keyword
pub fn r#type(args: &Vec<String>, mut output_config: OutputConfig) -> ControlFlow<()> {
    for arg in args {
        match BuiltinCommand::try_from(arg.clone()) {
            Ok(_) => {
                writeln!(output_config.stdout, "{} is a shell builtin", arg).unwrap();
            }
            Err(_) => {
                let finder = PathFinder::new(arg.clone());
                match finder.find_executable() {
                    Some(path) => {
                        writeln!(output_config.stdout, "{} is {}", arg, path.display()).unwrap()
                    }
                    None => writeln!(output_config.stderr, "{}: not found", arg).unwrap(),
                }
            }
        }
    }
    ControlFlow::Continue(())
}

pub fn pwd(mut output_config: OutputConfig) -> ControlFlow<()> {
    let path = std::env::current_dir().expect("couldn't access current working directory");
    writeln!(output_config.stdout, "{}", path.display()).unwrap();
    ControlFlow::Continue(())
}

pub fn cd(args: &[String]) -> Result<ControlFlow<()>, NotFound> {
    let home = env::home_dir()
        .expect("couldn't get path of current user's HOME directory")
        .to_string_lossy()
        .into();
    let path = if let Some(p) = args.first() {
        if p == "~" { home } else { p.clone() }
    } else {
        home
    };
    env::set_current_dir(&path)?;
    Ok(ControlFlow::Continue(()))
}

pub fn executable(
    path: &Path,
    args: &Vec<String>,
    mut output_config: OutputConfig,
) -> ControlFlow<()> {
    let command_out = std::process::Command::new(path.file_name().unwrap())
        .args(args)
        .output()
        .unwrap();
    output_config.stdout.write_all(&command_out.stdout).unwrap();
    output_config.stderr.write_all(&command_out.stderr).unwrap();
    ControlFlow::Continue(())
}