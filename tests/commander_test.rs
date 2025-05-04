use nost::commander::{handle_command, print_usage};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_print_usage() {
    // Capture the output of print_usage
    let output = std::panic::catch_unwind(|| {
        print_usage();
    });

    assert!(output.is_ok());
}

#[test]
fn test_handle_command_stats() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("file.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "Hello, world!").unwrap();

    let args = vec!["program".to_string(), "stats".to_string()];
    let not_path = temp_dir.path().to_str().unwrap();

    let result = handle_command(&args, not_path);
    assert!(result.is_ok());
}

#[test]
fn test_handle_command_extract() {
    let temp_dir = tempdir().unwrap();
    let args = vec![
        "program".to_string(),
        "extract".to_string(),
        "keyword".to_string(),
    ];
    let not_path = temp_dir.path().to_str().unwrap();

    let result = handle_command(&args, not_path);
    assert!(result.is_ok());
}

#[test]
fn test_handle_command_append() {
    let temp_dir = tempdir().unwrap();
    let args = vec!["program".to_string(), "append".to_string()];
    let not_path = temp_dir.path().to_str().unwrap();

    let result = handle_command(&args, not_path);
    assert!(result.is_ok());
}

#[test]
fn test_handle_command_unknown() {
    let temp_dir = tempdir().unwrap();
    let args = vec!["program".to_string(), "unknown".to_string()];
    let not_path = temp_dir.path().to_str().unwrap();

    let result = handle_command(&args, not_path);
    assert!(result.is_ok());
}
