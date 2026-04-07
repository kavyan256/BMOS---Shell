use std::io::{self, Write};

fn main() {
     
    loop{ 
        //print the prompt
        print!("$ ");

        //because rust doesnt automatically flush stdout, we need to do it ourselves
        io::stdout().flush().unwrap();

        //taking in the user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        println!("{}: command not found", input.trim());
    }
}