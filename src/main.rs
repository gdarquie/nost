mod commands;
mod files;

use commands::{append, compute_stats, list_ideas, not_film_viewing, print_commands, start_idea};
use dotenv::dotenv;
use std::env;
use std::io;

fn main() -> io::Result<()> {
    dotenv().ok();

    let not_path = env::var("NOT_PATH").expect("NOT_PATH must be set in the .env file");
    let files_limit = env::var("FILES_LIMIT")
        .unwrap_or_else(|_| "100000".to_string())
        .parse::<usize>()
        .expect("FILES_LIMIT must be a number");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_commands();
        return Ok(());
    }

    match args[1].as_str() {
        "film" => not_film_viewing(&not_path),
        "idea" => {
            if args.len() < 3 {
                return list_ideas();
            }
            let idea = &args[2];
            println!("Idea: {}", idea);
            start_idea(&not_path, &files_limit)
        }
        "stats" => compute_stats(&not_path, &files_limit),
        "append" => append(&not_path, &files_limit),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Use 'cargo run' without arguments to see available commands.");
            Ok(())
        }
    }
}
