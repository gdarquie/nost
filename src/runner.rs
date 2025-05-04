use crate::path_getter::get_not_files_pathes;
use std::{fs, io};

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
