use nost::{get_files_from_path, get_folders_pathes, get_parent_folders_pathes};
use std::env;
use std::path::PathBuf;
use std::{fs, io};

const FILES_LIMIT: usize = 100000;
const NOT_PATH: &str = "/home/gaetan/not";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <command> [options]");
        eprintln!("Available commands:");
        eprintln!("  stats                Get stats from not files");
        eprintln!("  extract <keyword>    Extract content with a specific keyword");
        eprintln!("  append               Append content to a file");
        return Ok(());
    }

    match args[1].as_str() {
        "stats" => run_stats(),
        "extract" => {
            if args.len() < 3 {
                eprintln!("Usage: cargo run extract <keyword>");
                return Ok(());
            }
            run_extract(&args[2])
        }
        "append" => run_append(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Use 'cargo run' without arguments to see available commands.");
            Ok(())
        }
    }
}

fn run_stats() -> io::Result<()> {
    let all_files = get_not_files_pathes()?;

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

fn run_extract(keyword: &str) -> io::Result<()> {
    println!("Extracting content with keyword: {}", keyword);
    // TODO: Implement extraction logic

    // use lib and get all folders then files
    // for each file, check if the keyword is in the file
    // if it is, get the last extract and append it
    // to the last not file
    Ok(())
}

fn run_append() -> io::Result<()> {
    println!("Appending content to the last `.md` file in the not folder...");

    // Find all `.md` files in the NOT_PATH directory
    let mut md_files = get_not_files_pathes()?;

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

fn get_not_files_pathes() -> io::Result<Vec<PathBuf>> {
    let mut parent_folders = fs::read_dir(NOT_PATH)?
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
