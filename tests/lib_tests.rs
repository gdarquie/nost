#[cfg(test)]
mod tests {
    use nost::{
        get_files_from_path, get_folders_pathes, get_not_files_pathes, get_parent_folders_pathes,
        handle_command, run_append, run_extract, run_stats,
    };
    use std::fs::{self, File};
    use std::io::{Read, Write};
    use tempfile::tempdir;

    #[test]
    fn test_get_parent_folders_pathes() {
        let temp_dir = tempdir().unwrap();
        let valid_folder = temp_dir.path().join("123");
        let invalid_folder = temp_dir.path().join("abc");

        fs::create_dir(&valid_folder).unwrap();
        fs::create_dir(&invalid_folder).unwrap();

        let parent_folders = vec![valid_folder.clone(), invalid_folder.clone()];
        let result = get_parent_folders_pathes(parent_folders);

        assert_eq!(result, vec![valid_folder]);
    }

    #[test]
    fn test_get_folders_pathes() {
        let temp_dir = tempdir().unwrap();
        let sub_folder = temp_dir.path().join("subfolder");
        fs::create_dir(&sub_folder).unwrap();

        let result = get_folders_pathes(temp_dir.path().to_path_buf()).unwrap();
        assert!(result.contains(&temp_dir.path().to_path_buf()));
        assert!(result.contains(&sub_folder));
    }

    #[test]
    fn test_get_files_from_path() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("file.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        let result = get_files_from_path(temp_dir.path().to_path_buf()).unwrap();
        assert!(result.contains(&file_path));
    }

    #[test]
    fn test_get_not_files_pathes() {
        let temp_dir = tempdir().unwrap();
        let folder_path = temp_dir.path().join("folder");
        let file_path = folder_path.join("file.md");

        fs::create_dir(&folder_path).unwrap();
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        let result = get_not_files_pathes(temp_dir.path().to_str().unwrap()).unwrap();
        assert!(result.contains(&file_path));
    }

    #[test]
    fn test_run_stats() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("file.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        let result = run_stats(temp_dir.path().to_str().unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_extract() {
        let result = run_extract("keyword");
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_append() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("file.md");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Initial content").unwrap();

        let result = run_append(temp_dir.path().to_str().unwrap());
        assert!(result.is_ok());

        let mut file_content = String::new();
        File::open(&file_path)
            .unwrap()
            .read_to_string(&mut file_content)
            .unwrap();

        assert!(file_content.contains("Initial content"));
        assert!(file_content.contains("Append content to the last not file"));
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
    fn test_handle_command_append() {
        let temp_dir = tempdir().unwrap();
        let args = vec!["program".to_string(), "append".to_string()];
        let not_path = temp_dir.path().to_str().unwrap();

        let result = handle_command(&args, not_path);
        assert!(result.is_ok());
    }
}
