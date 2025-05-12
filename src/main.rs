use dotenv::dotenv;
use nost::commands::{append, compute_stats, extract, print_commands};
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
        "stats" => compute_stats(&not_path, &files_limit),
        "extract" => {
            if args.len() < 3 {
                eprintln!("Usage: cargo run extract <keyword>");
                return Ok(());
            }
            extract(&args[2])
        }
        "append" => append(&not_path, &files_limit),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Use 'cargo run' without arguments to see available commands.");
            Ok(())
        }
    }
}
