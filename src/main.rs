use regex::Regex;
use std::path::PathBuf;
use std::{fs, io};

const FOLDER_LIMIT: usize = 1000;

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
        if folders.len() >= FOLDER_LIMIT {
            return Err(format!("Folder limit exceeded ({})", FOLDER_LIMIT));
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

fn main() -> io::Result<()> {
    const NOT_PATH: &str = "/home/gaetan/not";

    let mut parent_folders = fs::read_dir(NOT_PATH)?
        .map(|parent_folder| parent_folder.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    parent_folders.sort();

    let parent_int_folders: Vec<PathBuf> = get_parent_folders_pathes(parent_folders);

    let all_folders: Vec<PathBuf> = parent_int_folders
        .iter()
        .flat_map(|folder| get_folders_pathes(folder.clone()).unwrap_or_default())
        .collect();

    // create a list of all the files in this folders

    // parse all this files

    all_folders
        .iter()
        .for_each(|entry| println!("An entry {}", entry.display()));
    Ok(())
}
