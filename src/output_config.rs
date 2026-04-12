use crate::error::not_found::NotFound;
use crate::output::Output;
use std::{
    fs::{File, OpenOptions},
    io,
    path::PathBuf,
};

//The OutputConfig struct is responsible for configuring the output of a command, allowing for redirection 
//of stdout and stderr to files. It provides a way to specify whether the output should be written to the 
//console (stdout or stderr) or redirected to a file, and whether the file should be overwritten or appended to. 
//This is essential for implementing features like output redirection in a shell.

pub struct OutputConfig {
    pub stdout: Output,
    pub stderr: Output,
}

//implementing the Default trait for OutputConfig to provide a default configuration 
impl Default for OutputConfig {
    fn default() -> Self {
        OutputConfig {
            stdout: Output::StdOut(io::stdout()),
            stderr: Output::StdErr(io::stderr()),
        }
    }
}

// Configure output redirection (>, >>, 2>, 2>>) for stdout/stderr to files
impl OutputConfig {
    pub fn new(symbol: &str, file_path: PathBuf) -> Result<Self, NotFound> {
        match symbol {
            ">" | "1>" => Ok(OutputConfig {
                stdout: Output::File(File::create(file_path)?),
                stderr: Output::StdErr(io::stderr()),
            }),

            "2>" => Ok(OutputConfig {
                stdout: Output::StdOut(io::stdout()),
                stderr: Output::File(File::create(file_path)?),
            }),
            ">>" | "1>>" => Ok(OutputConfig {
                stdout: Output::File(
                    OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(file_path)?,
                ),
                stderr: Output::StdErr(io::stderr()),
            }),
            "2>>" => Ok(OutputConfig {
                stdout: Output::StdOut(io::stdout()),
                stderr: Output::File(
                    OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(file_path)?,
                ),
            }),
            _ => Err(NotFound::OutputConfigSymbol),
        }
    }
}