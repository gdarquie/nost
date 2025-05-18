use crate::files::get_files_from_path;
use std::{fs, io};

pub fn print_commands() {
    eprintln!("Available commands:");
    eprintln!("  stats                Get stats from not files");
    eprintln!("  extract <keyword>    Extract content with a specific keyword");
    eprintln!("  append               Append content to a file");
}

pub fn compute_stats(not_path: &str, files_limit: &usize) -> io::Result<()> {
    let all_files = get_files_from_path((&not_path).into());

    match all_files {
        Ok(files) => {
            for file in files {
                let lines = fs::read_to_string(&file).unwrap_or_default();
                let line_count = lines.lines().count();
                let word_count = lines.split_whitespace().count();

                println!(
                    "File: {} - Number of lines: {}",
                    &file.display(),
                    line_count
                );
                println!(
                    "File: {} - Number of words: {}",
                    &file.display(),
                    word_count
                );
            }
        }
        Err(err) => {
            eprintln!("Error retrieving files: {}", err);
        }
    }

    Ok(())
}

pub fn extract(keyword: &str) -> io::Result<()> {
    println!("Extracting content with keyword: {}", keyword);
    // TODO: Implement extraction logic

    // use lib and get all folders then files
    // for each file, check if the keyword is in the file
    // if it is, get the last extract and append it
    // to the last not file
    Ok(())
}

pub fn append(not_path: &str, files_limit: &usize) -> io::Result<()> {
    println!("Appending content to the last `.md` file in the not folder...");

    // Find all `.md` files in the NOT_PATH directory
    let mut md_files = get_not_files_pathes(not_path, files_limit)?;

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

        writeln!(file, "[//]: # \"extract-start-test\"")?;
        writeln!(file, "\n")?;
        writeln!(file, "[//]: # \"extract-end-test\"")?;
        println!(
            "Content appended successfully to {}",
            last_md_file.display()
        );
    } else {
        println!("No `.md` files found in the not folder.");
    }

    Ok(())
}
