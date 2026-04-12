mod argument_parser;
mod builtin_command;
mod error;
mod input;
mod order;
mod output;
mod output_config;
mod path_finder;
mod runner;
mod shell;
mod check_builtin;
mod completion;

use std::ops::ControlFlow;
//use std::io::{self, Write};
use rustyline::Editor;
use crate::completion::ShellHelper;

fn main() -> rustyline::Result<()> {
    let helper = ShellHelper::new();
    let mut rl = Editor::new()?;
    rl.set_helper(Some(helper));

    loop {
        let line = rl.readline("$ ")?;

        let Some(order) = shell::input(line) else {
            continue;
        };

        match order.execute() {
            ControlFlow::Continue(_) => continue,
            ControlFlow::Break(_) => break Ok(()),
        }
    }
}