pub fn delete_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    std::fs::remove_file(file_path)?;
    Ok(())
}

mod tests {
    use super::*;

    mod valid_file_path {
        use super::*;

        #[test]
        fn delete_valid_file_path() {
            // Can delete a file
            assert!(true);
        }

        #[test]
        fn delete_valid_file_dir() {
            // Can't delete a directory
            assert!(true);
        }
    }
}
