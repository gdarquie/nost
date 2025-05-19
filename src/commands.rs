use crate::files::get_files_from_path;
use std::fs::OpenOptions;
use std::io::Write;
use std::{fs, io};

pub fn print_commands() {
    eprintln!("Available commands:");
    eprintln!("  stats                Get stats from not files");
    eprintln!("  extract <keyword>    Extract content with a specific keyword");
    eprintln!("  append               Append content to a file");
}

pub fn compute_stats(not_path: &str, _files_limit: &usize) -> io::Result<()> {
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

pub fn append(not_path: &str, _files_limit: &usize) -> io::Result<()> {
    println!("Appending content to the last `.md` file in the not folder...");

    // Find all `.md` files in the NOT_PATH directory
    let md_files = match get_files_from_path(not_path.into()) {
        Ok(files) => files,
        Err(err) => {
            eprintln!("Error retrieving files: {}", err);
            return Ok(());
        }
    };

    // Get the last `.md` file
    if let Some(last_md_file) = md_files.last() {
        println!("Found last `.md` file: {}", last_md_file.display());

        // Append content to the file
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(last_md_file)?;

        writeln!(file, "\n")?;
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

pub fn start_idea(not_path: &str, _files_limit: &usize) -> io::Result<()> {
    println!("Starting idea...");
    // if idea already exists, we could think to add a log to say it
    // append the idea to the last file
    let not_files = match get_files_from_path(not_path.into()) {
        Ok(files) => files,
        Err(err) => {
            eprintln!("Error retrieving files: {}", err);
            return Ok(());
        }
    };

    if let Some(last_md_file) = not_files.last() {
        println!("Found last `.md` file: {}", last_md_file.display());

        // Append content to the file
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(last_md_file)?;

        writeln!(file, "\n")?;
        writeln!(file, "## [nost-idea]")?;
        writeln!(file, "\n")?;

        println!(
            "Content appended successfully to {}",
            last_md_file.display()
        );
    } else {
        println!("No `.md` files found in the not folder.");
    }

    // let mut file = OpenOptions::new().write(true).append(true).open(last_not)?;
    // writeln!(file, "\n")?;
    // writeln!(file, "[//]: # \"extract-start-test\"")?;
    // writeln!(file, "\n")?;
    // writeln!(file, "[//]: # \"extract-end-test\"")?;
    // println!(
    //     "Content appended successfully to {}",
    //     last_md_file.display()
    // );

    Ok(())
}

pub fn list_ideas() -> io::Result<()> {
    println!("List existing ideas...");
    Ok(())
}
