use crate::output_config::OutputConfig;
use crate::runner;
use crate::builtin_command::BuiltinCommand;
use crate::check_builtin::Command;
use std::ops::ControlFlow;

// Order file acts as the manager of the execution of cmds. It takes in a cmd and its args along with the output
// configuration and executes the cmd accordingly, handling any errors that may arise during execution.

pub struct Order {
    command: Command,
    args: Vec<String>,
    output_config: OutputConfig,
}

impl Order {

    //constructor
    pub fn new(command: Command, args: Vec<String>, output_config: OutputConfig) -> Self {
        Order {
            command,
            args,
            output_config,
        }
    }

    //execute the command based on whether it's a builtin or an executable,and handles errors
    pub fn execute(self) -> ControlFlow<()> {
        let Self {
            command,
            args,
            output_config,
        } = self;

        //matches for the cmd and executes respective runner
        let result = match &command {
            Command::Builtin(BuiltinCommand::Exit) => Ok(runner::exit()),
            Command::Builtin(BuiltinCommand::Echo) => Ok(runner::echo(&args, output_config)),
            Command::Builtin(BuiltinCommand::TypeCmd) => Ok(runner::r#type(&args, output_config)),
            Command::Builtin(BuiltinCommand::Pwd) => Ok(runner::pwd(output_config)),
            Command::Builtin(BuiltinCommand::Cd) => runner::cd(&args),
            Command::Executable(path) => Ok(runner::executable(path, &args, output_config)),
        };

        //checks result and handles errors by printing and REPL
        match result {
            Ok(control_flow) => control_flow,
            Err(err) => {
                eprintln!("{}: {}: {}", command, args.join(" "), err);
                ControlFlow::Continue(())
            }
        }
    }
}