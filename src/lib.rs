use regex::Regex;
use std::path::PathBuf;
use std::{fs, io};

const FOLDERS_LIMIT: usize = 1000;

pub fn get_parent_folders_pathes(parent_folders: Vec<PathBuf>) -> Vec<PathBuf> {
    let re = Regex::new(r"^\d+$").unwrap(); // Matches strings that are only digits

    return parent_folders
        .iter()
        .filter(|folder| {
            folder
                .file_name()
                .and_then(|name| name.to_str())
                .map(|name_str| re.is_match(name_str))
                .unwrap_or(false)
        })
        .cloned()
        .collect();
}

pub fn get_folders_pathes(folder_path: PathBuf) -> Result<Vec<PathBuf>, String> {
    let mut folders = Vec::new();
    let mut stack = vec![folder_path];

    while let Some(current) = stack.pop() {
        if folders.len() >= FOLDERS_LIMIT {
            return Err(format!("Folder limit exceeded ({})", FOLDERS_LIMIT));
        }

        if current.is_dir() {
            folders.push(current.clone());

            match fs::read_dir(&current) {
                Ok(entries) => {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() {
                            stack.push(path);
                        }
                    }
                }
                Err(err) => eprintln!("Failed to read {}: {}", current.display(), err),
            }
        }
    }

    Ok(folders)
}

pub fn get_files_from_path(path: PathBuf) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    let mut stack = vec![path];

    while let Some(current) = stack.pop() {
        if current.is_dir() {
            match fs::read_dir(&current) {
                Ok(entries) => {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() {
                            stack.push(path);
                        } else {
                            files.push(path);
                        }
                    }
                }
                Err(err) => eprintln!("Failed to read {}: {}", current.display(), err),
            }
        }
    }

    Ok(files)
}
