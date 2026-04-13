// Filename/path completion — to be implemented.
// Called when the cursor is on an argument position (position >= 1).

// Returns filesystem candidates matching the given partial path prefix.
// Examples:
//   "src/"   → files and dirs inside src/
//   "~/Doc"  → expands ~ and matches inside home dir
//   "fo"     → matches "foo/", "foo.txt" in current directory
pub fn complete_paths(_word: &str) -> Vec<String> {
    // TODO: implement filename completion
    // Hints for implementation:
    // 1. Expand ~ to home dir (std::env::home_dir or $HOME)
    // 2. Split word into (parent_dir, partial_name) — e.g. "src/ma" → ("src/", "ma")
    // 3. Read entries of parent_dir with std::fs::read_dir
    // 4. Filter entries whose filename starts with partial_name
    // 5. Append "/" for directories, " " for files
    Vec::new()
}