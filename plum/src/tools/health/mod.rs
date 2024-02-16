use core::panic;
use serde_json::Value;
use std::{path::PathBuf, process::Command, vec};

/*
* Returns the workspace root as a slice of the path
 */
pub fn find_workspace_root() -> PathBuf {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version=1", "--no-deps"])
        .output()
        .unwrap();

    if !output.status.success() {
        panic!("Failed to run cargo commands in the project. To debug, run `cargo metadata --format-version=1 --no-deps` from the same path.");
    }

    let metadata = serde_json::from_slice::<Value>(&output.stdout).unwrap();
    let workspace_root = metadata["workspace_root"].as_str().unwrap();
    PathBuf::from(workspace_root)
}

pub fn is_workspace_configured() -> bool {
    let workspace_root = find_workspace_root();

    // Step 1: check if the workspace root contains a Cargo.toml file

    // Step 2: check if the workspace root contains a plum directory with a correct Cargo.toml file

    // Step 3: check if the workspace root contains a plugins directory that matches all the plugin names as stated in the workspace root's plum/Cargo.toml file

    true
}

/* 3 functions: return boolean to state the health of specified configuration space */
fn is_root_configured() -> bool {
    true
}

fn is_core_configured() -> bool {
    true
}

fn is_plugins_configured() -> bool {
    true
}

/* 3 functions: returns the tools and plugins from the project configuration */
fn get_plugins_from_workspace_root() -> Vec<String> {
    Vec::new()
}

fn get_plugins_from_plugins_dir() -> Vec<String> {
    Vec::new()
}

fn get_core_tools() -> Vec<String> {
    Vec::new()
}

/* Used to */
fn array_diff<T: PartialEq + Clone>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
    let mut diff: Vec<T> = Vec::new();
    for item in vec1.iter() {
        if !vec2.contains(item) {
            diff.push(item.clone());
        }
    }
    diff
}
