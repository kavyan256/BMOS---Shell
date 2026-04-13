// Filename completion — searches the current directory for files matching the given prefix.
// Appends a trailing space to the completed filename.
pub fn complete_paths(word: &str) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(".") else {
        return Vec::new();
    };

    entries
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with(word) {
                Some(format!("{} ", name))
            } else {
                None
            }
        })
        .collect()
}