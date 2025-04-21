#[cfg(test)]
mod tests {
    use nost::{
        append_to_file, get_files_from_path, get_folders_pathes, get_parent_folders_pathes,
    };
    use std::fs::{self, File};
    use std::io::Read;
    use std::io::Write;
    use tempfile::{tempdir, NamedTempFile};

    #[test]
    fn test_append_to_file() {
        // Create a temporary file
        let temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_str().unwrap();

        // Write initial content to the file
        writeln!(&temp_file, "Initial content").unwrap();

        // Append new content to the file
        append_to_file(temp_path, "Appended content").unwrap();

        // Read the file content
        let mut file_content = String::new();
        fs::File::open(temp_path)
            .unwrap()
            .read_to_string(&mut file_content)
            .unwrap();

        // Assert that the file contains both the initial and appended content
        assert!(file_content.contains("Initial content"));
        assert!(file_content.contains("Appended content"));
    }

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
}
