use crate::input;
use crate::order::Order;

//takes in a line of input from the user and parses it into an Order
pub fn input(line: String) -> Option<Order> {
    //trim whitespace and newlines from input
    let line = line.trim().to_string();

    //if the input is empty
    if line.is_empty() {
        return None;
    }

    //parse the input into an order 
    let parsed_input = input::parse_input(line);

    //check if the input is valid and return an order
    match parsed_input {    
        Ok((command, args, output_conf)) => Some(Order::new(command, args, output_conf)),
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}