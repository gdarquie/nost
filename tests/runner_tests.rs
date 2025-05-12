use nost::commands::{run_append, run_extract, run_stats};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

// #[test]
// fn test_run_append() {
//     let temp_dir = tempdir().unwrap();
//     let file_path = temp_dir.path().join("file.md");
//     let mut file = File::create(&file_path).unwrap();
//     writeln!(file, "Initial content").unwrap();

//     // Call run_append to append content to the file
//     let result = run_append(temp_dir.path().to_str().unwrap());
//     assert!(result.is_ok());

//     // Verify that the content was appended
//     let mut file_content = String::new();
//     File::open(&file_path)
//         .unwrap()
//         .read_to_string(&mut file_content)
//         .unwrap();

//     assert!(file_content.contains("Initial content"));
//     assert!(file_content.contains("Append content to the last not file"));
// }
#[test]
fn test_run_extract() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("file.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "This is a test file with the keyword").unwrap();

    // Call run_extract with a keyword
    let result = run_extract("keyword");
    assert!(result.is_ok());

    // Verify that the function executed successfully
    // (You may need to adjust this based on the actual behavior of run_extract)
}

#[test]
fn test_run_stats() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("file.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "Hello, world!").unwrap();

    // Call run_stats to get stats for the file
    let result = run_stats(temp_dir.path().to_str().unwrap(), 1000);
    assert!(result.is_ok());

    // Verify that the function executed successfully
    // (You may need to adjust this based on the actual behavior of run_stats)
}
