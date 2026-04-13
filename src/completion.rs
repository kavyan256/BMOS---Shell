use rustyline::completion::Completer;               //logic for autocompletion of commands and paths
use rustyline::highlight::Highlighter;              //logic for syntax highlighting of commands and paths
use rustyline::hint::Hinter;                        //logic for providing hints for commands and paths  
use rustyline::validate::Validator;                 //logic for validating the input command before execution
use rustyline::Helper;                              //trait that combines Completer, Highlighter, etc into a single helper struct for the REPL
use crate::builtin_command::BuiltinCommand;         //builtin commands enum

pub struct ShellHelper;

impl ShellHelper {
    pub fn new() -> Self {
        ShellHelper
    }
}

impl Completer for ShellHelper {
    type Candidate = String;

    // Called on Tab: given the current line and cursor position, returns
    // (start_pos, candidates) where start_pos is where the replacement begins.
    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        // Only complete the first word (the command itself).
        // If the user has already typed a space, there's an argument — skip for now.
        let word = &line[..pos];
        if word.contains(' ') {
            return Ok((pos, Vec::new()));
        }

        // Collect every builtin that starts with what the user has typed so far.
        let matches: Vec<String> = BuiltinCommand::variants()
            .iter()
            .filter(|&&cmd| cmd.starts_with(word))
            .map(|&cmd| {
                // Append a space so the cursor lands after the completed command,
                // ready for the user to type arguments.
                format!("{} ", cmd)
            })
            .collect();

        if matches.is_empty() {
            // Ring the bell to signal nothing matched.
            print!("{}", '\x07');
            Ok((0, Vec::new()))
        } else {
            // Replace from position 0 so the whole partial word is swapped out.
            Ok((0, matches))
        }
    }
}

impl Hinter for ShellHelper {
    type Hint = String;

    // Show a greyed-out hint for the first matching builtin as the user types.
    // This gives beginners a preview of what Tab will complete to.
    fn hint(&self, line: &str, pos: usize, _ctx: &rustyline::Context) -> Option<Self::Hint> {
        if line.is_empty() || line.contains(' ') {
            return None;
        }

        let word = &line[..pos];
        BuiltinCommand::variants()
            .iter()
            .find(|&&cmd| cmd.starts_with(word) && cmd != word)
            .map(|&cmd| cmd[word.len()..].to_string()) // show only the missing suffix
    }
}

impl Highlighter for ShellHelper {}

impl Validator for ShellHelper {}

impl Helper for ShellHelper {}