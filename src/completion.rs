use rustyline::completion::Completer;               //logic for autocompletion of commands and paths
use rustyline::highlight::Highlighter;              //logic for syntax highlighting of commands and paths
use rustyline::hint::Hinter;                        //logic for providing hints for commands and paths  
use rustyline::validate::Validator;                 //logic for validating the input command before execution
use rustyline::Helper;                              //trait that combines Completer, Highlighter, etc into a single helper struct for the REPL
use crate::builtin_command::BuiltinCommand;         //builtin commands enum
use crate::path_finder::PathFinder;                 //logic for finding the full path of an executable command by searching through the directories listed in the PATH environment variable

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
        let word = &line[..pos];
        if word.contains(' ') {
            return Ok((pos, Vec::new()));
        }

        // Builtins first — they take priority over PATH executables.
        let mut matches: Vec<String> = BuiltinCommand::variants()
            .iter()
            .filter(|&&cmd| cmd.starts_with(word))
            .map(|&cmd| format!("{} ", cmd))
            .collect();

        // Extend with PATH executables, skipping any already covered by a builtin.
        let builtin_set: std::collections::HashSet<&str> = BuiltinCommand::variants()
            .iter()
            .copied()
            .collect();

        let mut exe_matches = PathFinder::find_executables_with_prefix(word)
            .into_iter()
            .filter(|name| !builtin_set.contains(name.as_str()))
            .map(|name| format!("{} ", name))
            .collect::<Vec<_>>();

        exe_matches.sort();
        matches.extend(exe_matches);

        // If no matches, ring the bell.
        if matches.is_empty() {
            print!("{}", '\x07');
            Ok((0, Vec::new()))
        } else {
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
        
        // First try to find a matching builtin hint
        if let Some(&cmd) = BuiltinCommand::variants()
            .iter()
            .find(|&&cmd| cmd.starts_with(word) && cmd != word)
        {
            let suffix = cmd[word.len()..].to_string();
            return Some(format!("\x1b[2m{}\x1b[0m", suffix));
        }

        // If no builtin matches, try to find an executable hint
        PathFinder::find_executables_with_prefix(word)
            .into_iter()
            .next()
            .and_then(|exe| {
                if exe.len() > word.len() {
                    let suffix = exe[word.len()..].to_string();
                    Some(format!("\x1b[2m{}\x1b[0m", suffix))
                } else {
                    None
                }
            })
    }
}

impl Highlighter for ShellHelper {}

impl Validator for ShellHelper {}

impl Helper for ShellHelper {}