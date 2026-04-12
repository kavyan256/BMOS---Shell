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

use std::ops::ControlFlow;
use std::io::{self, Write};

fn main() {
    loop {

        //print the shell prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        //take input from user
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        //parse the input into an order
        let Some(order) = shell::input(line) else {
            continue;
        };

        //execute the order
        match order.execute() {
            ControlFlow::Continue(_) => continue,
            ControlFlow::Break(_) => break,
        }
    }
}