use regex::Regex;
use std::fs;
use std::path::PathBuf;

pub fn get_files_from_path(path: PathBuf) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    let mut stack = vec![path];

    let folder_regex = Regex::new(r".*\d+$").unwrap();
    let file_regex = Regex::new(r".*\d+\.md$").unwrap();

    while let Some(current) = stack.pop() {
        match fs::read_dir(&current) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(folder_name) = path.file_name().and_then(|name| name.to_str()) {
                            if folder_regex.is_match(folder_name) {
                                stack.push(path);
                            }
                        }
                    } else {
                        if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                            if file_regex.is_match(file_name) {
                                files.push(path);
                            }
                        }
                    }
                }
            }
            Err(err) => return Err(format!("Failed to read directory: {}", err)),
        }
    }

    files.sort();

    Ok(files)
}
