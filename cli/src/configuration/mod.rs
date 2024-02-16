use serde_json::Value;
use std::{
    env::{current_dir, set_current_dir},
    error::Error,
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

    // Change the current directory to the workspace root before performing any operations.
    // let setdir_result = set_current_dir(&workspace_root);
    // if setdir_result.is_err() {
    //     panic!("Failed to set the current directory to the workspace root.");
    // }

    workspace_root
}

pub fn go_to_workspace_root() {}

pub fn check_if_plugin_exists(name: &str) -> bool {
    let workspace_root = find_workspace_root();
    let plugin_path = workspace_root.join("plugins").join("plugins").join(name);
    plugin_path.exists()
}
