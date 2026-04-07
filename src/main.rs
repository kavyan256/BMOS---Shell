use std::io::{self, Write};

fn main() {
     
    loop{ 
        //print the prompt
        print!("$ ");

        //because rust doesnt automatically flush stdout, we need to do it ourselves
        io::stdout().flush().unwrap();

        //taking in the user input
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        command = command.trim().to_string();
        
        //check for exit command
        if command == "exit" {
            break;
        }

        println!("{}: command not found", command.trim());
    }
}