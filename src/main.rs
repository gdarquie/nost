use dotenv::dotenv;
use nost::dispatcher::{dispatch, print_commands};
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

    dispatch(&args, &not_path, &files_limit)
}
