use dotenv::dotenv;
use nost::{run_append, run_extract, run_stats};
use std::env;
use std::io;

fn main() -> io::Result<()> {
    dotenv().ok();

    let not_path = env::var("NOT_PATH").expect("NOT_PATH must be set in the .env file");

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
        "stats" => run_stats(&not_path),
        "extract" => {
            if args.len() < 3 {
                eprintln!("Usage: cargo run extract <keyword>");
                return Ok(());
            }
            run_extract(&args[2])
        }
        "append" => run_append(&not_path),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Use 'cargo run' without arguments to see available commands.");
            Ok(())
        }
    }
}
