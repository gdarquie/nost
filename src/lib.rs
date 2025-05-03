use regex::Regex;
// use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};

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

pub fn get_not_files_pathes(not_path: &str) -> io::Result<Vec<PathBuf>> {
    const FILES_LIMIT: usize = 100000;

    let mut parent_folders = fs::read_dir(not_path)?
        .map(|parent_folder| parent_folder.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    parent_folders.sort();

    let parent_int_folders: Vec<PathBuf> = get_parent_folders_pathes(parent_folders);

    let all_folders: Vec<PathBuf> = parent_int_folders
        .iter()
        .flat_map(|folder| get_folders_pathes(folder.clone()).unwrap_or_default())
        .collect();

    let all_files: Vec<PathBuf> = all_folders
        .iter()
        .flat_map(|folder| get_files_from_path(folder.clone()).unwrap_or_default())
        .take(FILES_LIMIT)
        .collect();

    Ok(all_files)
}

pub fn run_stats(not_path: &str) -> io::Result<()> {
    let all_files = get_not_files_pathes(&not_path)?;

    println!("Number of files: {}", all_files.len());

    all_files.iter().for_each(|file| {
        let lines = fs::read_to_string(file).unwrap_or_default();
        let line_count = lines.lines().count();
        let word_count = lines.split_whitespace().count();

        println!("File: {} - Number of lines: {}", file.display(), line_count);
        println!("File: {} - Number of words: {}", file.display(), word_count);
    });

    Ok(())
}

pub fn run_extract(keyword: &str) -> io::Result<()> {
    println!("Extracting content with keyword: {}", keyword);
    // TODO: Implement extraction logic

    // use lib and get all folders then files
    // for each file, check if the keyword is in the file
    // if it is, get the last extract and append it
    // to the last not file
    Ok(())
}

pub fn run_append(not_path: &str) -> io::Result<()> {
    println!("Appending content to the last `.md` file in the not folder...");

    // Find all `.md` files in the NOT_PATH directory
    let mut md_files = get_not_files_pathes(not_path)?;

    // Sort the files based on the numeric representation of their paths
    md_files.sort_by_key(|path| {
        path.to_string_lossy()
            .replace("/", "") // Remove all slashes
            .replace(".md", "") // Remove the `.md` extension
            .parse::<u64>() // Parse the resulting string as a number
            .unwrap_or(0) // Use 0 if parsing fails
    });

    // Get the last `.md` file
    if let Some(last_md_file) = md_files.last() {
        println!("Found last `.md` file: {}", last_md_file.display());

        // Append content to the file
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(last_md_file)?;

        writeln!(file, "Append content to the last not file")?;
        println!(
            "Content appended successfully to {}",
            last_md_file.display()
        );
    } else {
        println!("No `.md` files found in the not folder.");
    }

    Ok(())
}

pub fn print_usage() {
    eprintln!("Available commands:");
    eprintln!("  stats                Get stats from not files");
    eprintln!("  extract <keyword>    Extract content with a specific keyword");
    eprintln!("  append               Append content to a file");
}

pub fn handle_command(args: &[String], not_path: &str) -> io::Result<()> {
    match args[1].as_str() {
        "stats" => run_stats(not_path),
        "extract" => {
            if args.len() < 3 {
                eprintln!("Usage: cargo run extract <keyword>");
                return Ok(());
            }
            run_extract(&args[2])
        }
        "append" => run_append(not_path),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Use 'cargo run' without arguments to see available commands.");
            Ok(())
        }
    }
}
