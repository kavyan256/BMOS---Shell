use rustyline::completion::Completer;               //logic for autocompletion of commands and paths
use rustyline::highlight::Highlighter;              //logic for syntax highlighting of commands and paths
use rustyline::hint::Hinter;                        //logic for providing hints for commands and paths  
use rustyline::validate::Validator;                 //logic for validating the input command before execution
use rustyline::Helper;                              //trait that combines Completer, Highlighter, etc into a single helper struct for the REPL

pub struct ShellHelper;                             //struct to implement Helper Trait for REPL

//trait to combine stuff into single helper struct for REPL
impl ShellHelper {
    pub fn new() -> Self {
        ShellHelper     
    }
}

//Implementing trait for ShellHelper to provide autocompletion, hints, validation, and highlighting for the REPL.
impl Completer for ShellHelper {
    type Candidate = String;

    //takes the currently edited line with cursor position and resturns the start position and completion candidates for autocompletion
    fn complete(&self, line: &str, pos: usize, _ctx: &rustyline::Context) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        if line == "ech" && pos == line.len() {
            Ok((0, vec!["echo ".to_string()]))
        } else if line == "exi" && pos == line.len() {
            Ok((0, vec!["exit".to_string()]))
        } else {
            print!("{}", '\x07');       //print the bell character to indicate no completions available
            Ok((0, Vec::new()))         //empty vector for no completions 
        }
    }
}

impl Hinter for ShellHelper {
    type Hint = String;

    //takes the currently edited line with cursor position and returns a hint for autocompletion
    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context) -> Option<Self::Hint> {
        None
    }   
}


impl Highlighter for ShellHelper {}

impl Validator for ShellHelper {}

impl Helper for ShellHelper {}