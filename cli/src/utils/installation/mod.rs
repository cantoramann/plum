use crate::utils::workspace_config;
use reqwest::blocking::Client;
use std::fs;
use std::process::Command;
use std::{
    error::Error,
    fs::File,
    io::{self, Write},
};

/*

* Installs the plugin from the given path to the plugins directory.
*/
pub fn add_plugin(remote_path: &str, filename: String) -> Result<(), Box<dyn Error>> {
    // a map of filename to url
    let mut plugin_map = std::collections::HashMap::new();
    plugin_map.insert(
        "obsidian".to_string(),
        workspace_config::find_workspace_root().join("obsidian.zip"),
    );

    // if the filename is not in the map, return an error
    if !plugin_map.contains_key(&filename) {
        return Err("Plugin not found".into());
    }

    // new
    let plugins_dir = workspace_config::find_plugins_dir();

    // Ensure the plugins directory exists.
    fs::create_dir_all(&plugins_dir)?;

    // Combine the directory path with the filename to get the target path.
    let file_path: std::path::PathBuf = plugins_dir.join(&filename);

    // new command to get into plugins/plugins directory
    let _ = Command::new("curl")
        .arg(remote_path)
        .current_dir(plugins_dir)
        .output()
        .unwrap();

    Ok(())
}
