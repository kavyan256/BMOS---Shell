use crate::builtin_command::BuiltinCommand;
use crate::path_finder::PathFinder;

// Returns all command candidates (builtins + PATH executables) that match the given prefix.
// Builtins come first, PATH executables follow sorted alphabetically.
pub fn complete_commands(word: &str) -> Vec<String> {
    let mut matches: Vec<String> = BuiltinCommand::variants()
        .iter()
        .filter(|&&cmd| cmd.starts_with(word))
        .map(|&cmd| format!("{} ", cmd))
        .collect();

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
    matches
}