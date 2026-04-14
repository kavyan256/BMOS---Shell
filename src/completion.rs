use rustyline::completion::Completer;               //logic for autocompletion of commands and paths
use rustyline::highlight::Highlighter;              //logic for syntax highlighting of commands and paths
use rustyline::hint::Hinter;                        //logic for providing hints for commands and paths
use rustyline::validate::Validator;                 //logic for validating the input command before execution
use rustyline::Helper;                              //trait that combines Completer, Highlighter, etc into a single helper struct for the REPL
use crate::builtin_command::BuiltinCommand;         //for hints
use crate::path_finder::PathFinder;                 //for hints
use crate::command_completer;                       //command name completion (builtins + PATH)
use crate::path_completer;                          //filename/path completion
use std::cell::Cell;                                //interior mutability for tab press tracking without &mut self
use std::io::Write;

pub struct ShellHelper {
    last_was_tab: Cell<bool>,   //tracks whether the previous keypress was also TAB
}

//implements constructor for ShellHelper struct
impl ShellHelper {
    pub fn new() -> Self {
        ShellHelper {
            last_was_tab: Cell::new(false),
        }
    }
}

//implement the Completer trait for ShellHelper
impl Completer for ShellHelper {

    //specify Candidate type as String
    type Candidate = String; 

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {         //usize is the index where completion starts
        let before_cursor = &line[..pos];                     //slices from start to cursor
        let tokens: Vec<&str> = before_cursor.split_whitespace().collect();     //splits before_cursor into tokens
        let is_typing_command = tokens.len() <= 1 && !before_cursor.ends_with(' '); 

        // Decide which completer to use based on cursor position.
        let (start, mut candidates) = if is_typing_command {
            // Position 0: completing the command name.
            let word = tokens.first().copied().unwrap_or("");
            (0, command_completer::complete_commands(word))
        } else {
            // Position >= 1: completing a filename/path argument.
            let word = if before_cursor.ends_with(' ') {
                ""                                          //starting a new argument
            } else {
                tokens.last().copied().unwrap_or("")        //mid-word on current argument
            };
            let start = pos - word.len();
            (start, path_completer::complete_paths(word))
        };

        // No matches — ring bell, leave line unchanged.
        if candidates.is_empty() {
            print!("{}", '\x07');
            self.last_was_tab.set(false);
            return Ok((start, Vec::new()));
        }

        // Single match — complete immediately.
        if candidates.len() == 1 {
            self.last_was_tab.set(false);
            return Ok((start, candidates));
        }

        // Multiple matches — different behaviour for commands vs paths.
        self.last_was_tab.set(true);

        if is_typing_command {
            // Command completion: advance to LCP on first Tab, list on second.
            let lcp = longest_common_prefix(&candidates);
            let typed_len = pos - start;                    //len of what's currently typed
            if lcp.trim_end().len() > typed_len {
                // LCP is longer than what's typed — advance to it silently.
                self.last_was_tab.set(false);
                return Ok((start, vec![lcp]));
            }
            // Already at LCP boundary — show full list.
            return Ok((start, candidates));
        }

        // Path completion: advance to LCP on first Tab, list on second Tab.
        let lcp = longest_common_prefix(&candidates);
        let typed_len = pos - start;
        if lcp.trim_end().len() > typed_len {
            // LCP is longer than what's typed — advance to it silently.
            self.last_was_tab.set(false);
            return Ok((start, vec![lcp]));
        }

        // Already at LCP boundary — list on this Tab.
        // Second Tab: print all matches on a new line separated by two spaces.
        // Directories keep their trailing "/", files have trailing space stripped for display.
        candidates.sort();
        let display: Vec<String> = candidates
            .iter()
            .map(|c| c.trim_end_matches(' ').to_string())
            .collect();
        print!("\n{}\n", display.join("  "));
        std::io::stdout().flush().ok();

        // Return the current word unchanged so rustyline redraws the prompt
        // with the input intact and the cursor in the right place.
        let current_word = &line[start..pos];
        Ok((start, vec![current_word.to_string()]))
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