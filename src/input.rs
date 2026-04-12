use crate::argument_parser::ArgumentParser;
use crate::check_builtin::Command;
use crate::error::not_found::NotFound;
use crate::output_config::OutputConfig;

//The parse_input function takes a raw input string, parses it into a command, its arguments, and an output 
//configuration for redirection. It uses the ArgumentParser to split the input into components, identifies the 
//command and its arguments, and checks for any special symbols that indicate output redirection. If such symbols 
//are found, it configures the OutputConfig accordingly. 

pub fn parse_input(input: String) -> Result<(Command, Vec<String>, OutputConfig), NotFound> {
    let parsed_input = ArgumentParser::new(input).parse(); 
    let (command_array, args_array) = parsed_input.split_at(1);
    let command_string = command_array[0].clone();

    let command = Command::try_from(command_string)?;   //converts cmd string to cmd enum variant and checks existense
    let mut args = args_array;                        //copies args to a mutable variable for further processing

    let output_config = if let Some((index, symbol)) = find_special_symbols(args) {
        let file_path = args
            .get(index + 1)
            .ok_or(NotFound::RedirectTargetFile)?
            .into();                                            //converts the file path argument to a PathBuf

        args = &args[0..index];                                 //trims the args to include only the cmd args

        OutputConfig::new(symbol, file_path)                    //sets output config based on symbol and file path
    } else {
        Ok(OutputConfig::default())                             //uses the default output config
    }?;

    Ok((command, args.to_vec(), output_config))                 //returns the parsed cmd, args and output config
}

//function to find special symbol in the args for output redirection and return its index and the symbol itself
fn find_special_symbols(args: &[String]) -> Option<(usize, &String)> {
    let special_symbols = [">", "1>", "2>", ">>", "1>>", "2>>"];

    args.iter()
        .enumerate()
        .find(|(_index, arg)| special_symbols.contains(&arg.as_str()))
}