use std::{
    env::{current_dir, set_current_dir},
    fs::File,
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

pub fn find_workspace_root() -> PathBuf {
    // Get the current directory.
    let current_dir = current_dir().unwrap();

    // Find the workspace root using `cargo metadata`.
    let output = Command::new("cargo")
        .args(["metadata", "--format-version=1", "--no-deps"])
        .current_dir(&current_dir) // Ensure that cargo is run in the current directory.
        .output()
        .unwrap();

    if !output.status.success() {
        panic!("Failed to run cargo commands in the project. To debug, run `cargo metadata --format-version=1 --no-deps` from the same path.");
    }

    // Parse the output to find the workspace root.
    let metadata = serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap();
    let workspace_root = PathBuf::from(metadata["workspace_root"].as_str().unwrap());

    // Validate the Plum workspace.
    let plumconfig = workspace_root.join(".plumconfig");
    let cargo_toml = workspace_root.join("Cargo.toml");

    if !plumconfig.exists() {
        panic!("Located the workspace but not a Plum environment. Please include .plumconfig if you removed it.");
    }

    if !cargo_toml.exists() || !cargo_toml.is_file() {
        panic!("The Cargo.toml with workspace configuration is missing.");
    }

    workspace_root
}

pub fn set_current_directory_env() {
    let workspace_root = find_workspace_root();
    let setdir_result = set_current_dir(&workspace_root);
    if setdir_result.is_err() {
        panic!("Failed to set the current directory to the workspace root.");
    }
}

pub fn find_plugins_root() -> PathBuf {
    find_workspace_root().join("plugins")
}

pub fn find_plugins_dir() -> PathBuf {
    find_plugins_root().join("plugins")
}

pub fn tidy_plugins() {
    let workspace_root = find_workspace_root();
    let plugins_dir = workspace_root.join("plugins").join("plugins");

    // clear __MACOSX dir
    let macosx_dir = plugins_dir.join("__MACOSX");
    if macosx_dir.exists() {
        _ = std::fs::remove_dir_all(macosx_dir);
    }

    // // clear .DS_Store file
    let ds_store = plugins_dir.join(".DS_Store");
    if ds_store.exists() {
        _ = std::fs::remove_file(ds_store);
    }

    // // clear .gitignore file
    let gitignore = plugins_dir.join(".gitignore");
    if gitignore.exists() {
        _ = std::fs::remove_file(gitignore);
    }

    // list all the filenames in the plugins directory
    let filenames = std::fs::read_dir(plugins_dir)
        .unwrap()
        .map(|res| res.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    // write all the filenames to plugins.config.plum. The first plugins/, not the second one.
    let mut config_file =
        File::create(workspace_root.join("plugins").join("plugins.config.plum")).unwrap();
    for filename in filenames {
        config_file
            .write_all(filename.to_str().unwrap().as_bytes())
            .unwrap();
        config_file.write_all(b"\n").unwrap();
    }
}

mod tests {

    use std::env;
    use std::fs;
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_find_workspace_root() {
        // Step 1: Set up the mock environment
        let workspace_parent_dir = tempdir().expect("failed to create temp dir");

        // create a cargo project from this directory using Command
        let _ = Command::new("cargo")
            .args(["new", "test_project"])
            .current_dir(&workspace_parent_dir)
            .output()
            .unwrap();

        // get into the test_project directory
        let workspace_dir = workspace_parent_dir.path().join("test_project");

        // create a .plumconfig file
        fs::File::create(workspace_dir.join(".plumconfig")).expect("failed to create .plumconfig");

        // Step 2: Change the current directory to the mock workspace
        assert!(env::set_current_dir(&workspace_dir).is_ok());

        // Step 3: Run the function under test
        let found_root = find_workspace_root();

        // Step 4: Assert the expected outcome
        assert_eq!(found_root, workspace_dir);

        // Step 5: Clean up happens automatically when `temp_dir` goes out of scope
    }
}
