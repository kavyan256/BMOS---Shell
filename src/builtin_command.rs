#[derive(Debug)]
pub enum BuiltinCommand {
    Cd,
    Echo,
    Exit,
    TypeCmd,
    Pwd,
}

//so the logic is as follows:
//1. define an enum for all the commands
//2. implement a function to parse a string into the enum, matching the first argument to the command name
//3. if the command is not recognized, return an unknown variant with the command name

//TryFrom is a trait that allows us to define how to convert from one type to another, 
//in this case from String to BuiltinCommand. It returns a Result, which can be Ok if 
//the conversion is successful or Err if it fails. This is useful for error handling 
//when we try to parse a command that may not be recognized.

//converts a string to vuiltin cmd
impl TryFrom<String> for BuiltinCommand {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "cd" => Ok(BuiltinCommand::Cd),
            "echo" => Ok(BuiltinCommand::Echo),
            "exit" => Ok(BuiltinCommand::Exit),
            "type" => Ok(BuiltinCommand::TypeCmd),
            "pwd" => Ok(BuiltinCommand::Pwd),
            _ => Err("Not a builtin command".to_string()),
        }
    }
}

// Implementing Display trait for BuiltinCommand to allow easy printing of the command names
//(basic) whenever someone tries to print, use the string representation
impl std::fmt::Display for BuiltinCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltinCommand::Exit => write!(f, "exit"),
            BuiltinCommand::Echo => write!(f, "echo"),
            BuiltinCommand::TypeCmd => write!(f, "type"),
            BuiltinCommand::Pwd => write!(f, "pwd"),
            BuiltinCommand::Cd => write!(f, "cd"),
        }
    }
}

//Display Trait is a standard trait in Rust that allows us to specify how a 
//type should be formatted when printed. By implementing the Display trait 
//for BuiltinCommand, we can define how each variant of the enum should be 
//represented as a string when we print it. This is useful for debugging and 
//for any situation where we want to output the command name in a user-friendly way.