use std::path::Path;

// Filename completion — searches for files matching the given prefix.
// Handles simple names ("re") and nested paths ("src/ma").
// Appends "/" for directories (allows continued completion), " " for files (ready for next argument).
pub fn complete_paths(word: &str) -> Vec<String> {
    // Split "src/ma" into search_dir="src/" and prefix="ma".
    // Plain "re" becomes search_dir="." and prefix="re".
    let (search_dir, prefix) = split_dir_and_prefix(word);

    //remains same, read the current directory and return list of entries
    let Ok(entries) = std::fs::read_dir(&search_dir) else {
        return Vec::new();
    };

    //Filter entries to those starting with prefix and reconstructing
    entries
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if !name.starts_with(prefix.as_str()) {
                return None;
            }
            
            // Reconstruct the full path as the user typed it, e.g. "src/main.rs "
            let dir_prefix = if search_dir == "." {
                String::new()
            } else {
                search_dir.clone()
            };
            
            // Append "/" for directories (allows continued completion), " " for files
            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
            let suffix = if is_dir { "/" } else { " " };
            Some(format!("{}{}{}", dir_prefix, name, suffix))
        })
        .collect()
}

// "src/ma"  → ("src/", "ma")
// "re"      → (".",    "re")
fn split_dir_and_prefix(word: &str) -> (String, String) {
    let path = Path::new(word);
    match path.parent() {
        Some(parent) if parent != Path::new("") => {
            let dir = format!("{}/", parent.to_string_lossy());
            let prefix = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            (dir, prefix)
        }
        _ => (".".to_string(), word.to_string()),
    }
}