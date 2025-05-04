use nost::commander::{dispatch, print_commands};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_print_commands() {
    let output = std::panic::catch_unwind(|| {
        print_commands();
    });

    assert!(output.is_ok());
}

#[test]
fn test_dispatch_stats() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("file.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "Hello, world!").unwrap();

    let args = vec!["program".to_string(), "stats".to_string()];
    let not_path = temp_dir.path().to_str().unwrap();

    let result = dispatch(&args, not_path);
    assert!(result.is_ok());
}

#[test]
fn test_dispatch_extract() {
    let temp_dir = tempdir().unwrap();
    let args = vec![
        "program".to_string(),
        "extract".to_string(),
        "keyword".to_string(),
    ];
    let not_path = temp_dir.path().to_str().unwrap();

    let result = dispatch(&args, not_path);
    assert!(result.is_ok());
}

#[test]
fn test_dispatch_append() {
    let temp_dir = tempdir().unwrap();
    let args = vec!["program".to_string(), "append".to_string()];
    let not_path = temp_dir.path().to_str().unwrap();

    let result = dispatch(&args, not_path);
    assert!(result.is_ok());
}

#[test]
fn test_dispatch_unknown() {
    let temp_dir = tempdir().unwrap();
    let args = vec!["program".to_string(), "unknown".to_string()];
    let not_path = temp_dir.path().to_str().unwrap();

    let result = dispatch(&args, not_path);
    assert!(result.is_ok());
}
