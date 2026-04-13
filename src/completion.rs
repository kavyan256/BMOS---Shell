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
    // - Single match  → completes immediately
    // - Multiple matches → prints all options and keeps the current word so the
    //   user can keep typing to narrow down (rustyline handles this when
    //   CompletionType::List is set in main.rs)
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

        if matches.is_empty() {
            print!("{}", '\x07');
            return Ok((0, Vec::new()));
        }

        // Single match: complete it immediately.
        if matches.len() == 1 {
            return Ok((0, matches));
        }

        // Multiple matches: compute the longest common prefix.
        let lcp = longest_common_prefix(&matches);
        let lcp_name_len = lcp.trim_end().len();

        if lcp_name_len > word.len() {
            // First Tab: advance the line to the LCP and stop.
            // The user can keep typing or hit Tab again to see the full list.
            return Ok((0, vec![lcp]));
        }

        // Second Tab (already at LCP boundary): show the full list.
        Ok((0, matches))
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
// Candidates carry a trailing space (e.g. "git ") — we compare including it
// so a sole exact match like ["git "] still returns "git " with the space.
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