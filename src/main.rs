use std::io::{self, Write};

fn main() {
    //print the prompt
    print!("$ ");

    //because rust doesnt automatically flush stdout, we need to do it ourselves
    io::stdout().flush().unwrap();
}