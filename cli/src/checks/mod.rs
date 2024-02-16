use serde_json::Value;
use std::{path::PathBuf, process::Command};

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

    let pb = PathBuf::from(workspace_root);
    let plumconfig = pb.join(".plumconfig");
    if !plumconfig.exists() {
        panic!("Located the workspace but not a Plum environment. Please include .plumconfig if you removed it.");
    }

    pb
}

pub fn check_if_plugin_exists(name: &str) -> bool {
    let workspace_root = find_workspace_root();
    let plugin_path = workspace_root.join("plugins").join("plugins").join(name);
    plugin_path.exists()
}
