use rustyline::completion::Completer;               //logic for autocompletion of commands and paths
use rustyline::highlight::Highlighter;              //logic for syntax highlighting of commands and paths
use rustyline::hint::Hinter;                        //logic for providing hints for commands and paths
use rustyline::validate::Validator;                 //logic for validating the input command before execution
use rustyline::Helper;                              //trait that combines Completer, Highlighter, etc into a single helper struct for the REPL
use crate::builtin_command::BuiltinCommand;         //for hints
use crate::path_finder::PathFinder;                 //for hints
use crate::command_completer;                       //command name completion (builtins + PATH)
use crate::path_completer;                          //filename/path completion (to be implemented)

pub struct ShellHelper;

impl ShellHelper {
    pub fn new() -> Self {
        ShellHelper
    }
}

impl Completer for ShellHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let before_cursor = &line[..pos];
        let tokens: Vec<&str> = before_cursor.split_whitespace().collect();
        let is_typing_command = tokens.len() <= 1 && !before_cursor.ends_with(' ');

        // Decide which completer to use based on cursor position.
        let (start, candidates) = if is_typing_command {
            // Position 0: completing the command name.
            let word = tokens.first().copied().unwrap_or("");
            (0, command_completer::complete_commands(word))
        } else {
            // Position >= 1: completing a filename/path argument.
            let word = if before_cursor.ends_with(' ') {
                ""                                                   //starting a new argument
            } else {
                tokens.last().copied().unwrap_or("")        //mid-word on current argument
            };
            let start = pos - word.len();
            (start, path_completer::complete_paths(word))
        };

        if candidates.is_empty() {
            print!("{}", '\x07');
            return Ok((start, Vec::new()));
        }

        if candidates.len() == 1 {
            return Ok((start, candidates));
        }

        // Multiple matches: advance to longest common prefix on first Tab,
        // show full list on second Tab.
        let lcp = longest_common_prefix(&candidates);
        let typed_len = pos - start;
        if lcp.trim_end().len() > typed_len {
            return Ok((start, vec![lcp]));
        }

        Ok((start, candidates))
    }
}

impl Hinter for ShellHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, _ctx: &rustyline::Context) -> Option<Self::Hint> {
        if line.is_empty() || line.contains(' ') {
            return None;
        }

        let word = &line[..pos];

        // Builtins take priority for hints.
        if let Some(&cmd) = BuiltinCommand::variants()
            .iter()
            .find(|&&cmd| cmd.starts_with(word) && cmd != word)
        {
            let suffix = cmd[word.len()..].to_string();
            return Some(format!("\x1b[2m{}\x1b[0m", suffix));
        }

        // Fall back to first matching PATH executable.
        PathFinder::find_executables_with_prefix(word)
            .into_iter()
            .next()
            .filter(|exe| exe.len() > word.len())
            .map(|exe| {
                let suffix = exe[word.len()..].to_string();
                format!("\x1b[2m{}\x1b[0m", suffix)
            })
    }
}

impl Highlighter for ShellHelper {}

impl Validator for ShellHelper {}

impl Helper for ShellHelper {}

// Returns the longest string that is a common prefix of all candidates.
fn longest_common_prefix(candidates: &[String]) -> String {
    let first = match candidates.first() {
        Some(s) => s,
        None => return String::new(),
    };

    let mut lcp_len = first.len();
    for candidate in &candidates[1..] {
        lcp_len = candidate
            .chars()
            .zip(first.chars())
            .take_while(|(a, b)| a == b)
            .count()
            .min(lcp_len);
    }

    first[..lcp_len].to_string()
}