use regex::Regex;
use std::path::PathBuf;
use std::{fs, io};

const FOLDERS_LIMIT: usize = 1000;
const FILES_LIMIT: usize = 100000;
const NOT_PATH: &str = "/home/gaetan/not";

fn get_parent_folders_pathes(parent_folders: Vec<PathBuf>) -> Vec<PathBuf> {
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

fn get_folders_pathes(folder_path: PathBuf) -> Result<Vec<PathBuf>, String> {
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

fn get_files_from_path(path: PathBuf) -> Result<Vec<PathBuf>, String> {
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

fn main() -> io::Result<()> {
    let mut parent_folders = fs::read_dir(NOT_PATH)?
        .map(|parent_folder| parent_folder.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    parent_folders.sort();

    let parent_int_folders: Vec<PathBuf> = get_parent_folders_pathes(parent_folders);

    let all_folders: Vec<PathBuf> = parent_int_folders
        .iter()
        .flat_map(|folder| get_folders_pathes(folder.clone()).unwrap_or_default())
        .collect();

    // put all files in a vector
    let all_files: Vec<PathBuf> = all_folders
        // todo: limit the number of files to 100000
        .iter()
        .flat_map(|folder| get_files_from_path(folder.clone()).unwrap_or_default())
        .take(FILES_LIMIT)
        .collect();

    // print the number of files
    println!("Number of files: {}", all_files.len());

    // print the number of lines and words by file
    all_files.iter().for_each(|file| {
        let lines = fs::read_to_string(file).unwrap_or_default();
        let line_count = lines.lines().count();
        let word_count = lines.split_whitespace().count();

        println!("File: {} - Number of lines: {}", file.display(), line_count);
        println!("File: {} - Number of words: {}", file.display(), word_count);
    });

    // todo: parse every files and extract some content

    Ok(())
}
