use dotenv::dotenv;
use nost::{handle_command, print_usage};
use std::env;
use std::io;

fn main() -> io::Result<()> {
    dotenv().ok();

    let not_path = env::var("NOT_PATH").expect("NOT_PATH must be set in the .env file");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    handle_command(&args, &not_path)
}
