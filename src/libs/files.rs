use regex::Regex;
use std::path::PathBuf;
use std::{fs, io};

fn get_files_pathes(parent_path: PathBuf) -> Result<Vec<PathBuf>, String> {
    let mut folders = Vec::new();

    // get the path of of all the folder containing at list one file
    match fs::read_dir(&parent_path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    folders.push(path);
                }
            }
        }
        Err(err) => eprintln!("Failed to read {}: {}", parent_path.display(), err),
    }

    println!("Folders: {:?}", folders);
    Ok(folders)

    // for all the folders pathes, get the path of all the files
}

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
        const FOLDERS_LIMIT: usize = 1000;

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

pub fn get_not_files_pathes(not_path: &str, files_limit: &usize) -> io::Result<Vec<PathBuf>> {
    let parent_folders = fs::read_dir(not_path)?
        .map(|parent_folder| parent_folder.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let all_folders: Vec<PathBuf> = get_parent_folders_pathes(parent_folders)
        .iter()
        .flat_map(|folder| get_folders_pathes(folder.clone()).unwrap_or_default())
        .collect();

    let mut all_files: Vec<PathBuf> = all_folders
        .iter()
        .flat_map(|folder| get_files_from_path(folder.clone()).unwrap_or_default())
        .take(*files_limit)
        .collect();

    all_files.sort();

    Ok(all_files)
}

// Unit tests for private functions
#[cfg(test)]
mod tests {
    use super::*; // Access private functions
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_get_files_from_path() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("file.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        let result = get_files_from_path(temp_dir.path().to_path_buf()).unwrap();
        assert!(result.contains(&file_path));
    }

    #[test]
    fn test_get_folders_pathes() {
        let temp_dir = tempdir().unwrap();
        let sub_folder = temp_dir.path().join("subfolder");
        fs::create_dir(&sub_folder).unwrap();

        let result = get_folders_pathes(temp_dir.path().to_path_buf()).unwrap();
        assert!(result.contains(&temp_dir.path().to_path_buf()));
        assert!(result.contains(&sub_folder));
    }

    #[test]
    fn test_get_parent_folders_pathes() {
        let temp_dir = tempdir().unwrap();
        let valid_folder = temp_dir.path().join("123");
        let invalid_folder = temp_dir.path().join("abc");

        fs::create_dir(&valid_folder).unwrap();
        fs::create_dir(&invalid_folder).unwrap();

        let parent_folders = vec![valid_folder.clone(), invalid_folder.clone()];
        let result = get_parent_folders_pathes(parent_folders);

        assert_eq!(result, vec![valid_folder]);
    }

    // #[test]
    // fn test_get_not_files_pathes() {
    //     let temp_dir = tempdir().unwrap();
    //     let folder_path = temp_dir.path().join("folder");
    //     let file_path = folder_path.join("file.md");

    //     fs::create_dir(&folder_path).unwrap();
    //     let mut file = File::create(&file_path).unwrap();
    //     writeln!(file, "Hello, world!").unwrap();

    //     let result = get_not_files_pathes(temp_dir.path().to_str().unwrap()).unwrap();
    //     assert!(result.contains(&file_path));
    // }
}
