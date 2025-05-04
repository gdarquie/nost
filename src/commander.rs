use crate::runner::{run_append, run_extract, run_stats};
use std::io;

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
