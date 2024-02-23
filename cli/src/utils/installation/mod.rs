use crate::utils::compression::decompress_package_in_dir;
use crate::utils::workspace_config;
use std::error::Error;
use std::fs;

pub fn add_plugin(remote_path: &str, filename: String) -> Result<(), Box<dyn Error>> {
    println!("Installing plugin from {}", remote_path);
    println!("Installing plugin {}", filename);

    // a map of filename to url - for now, we are using a local file
    let mut plugin_map = std::collections::HashMap::new();
    plugin_map.insert(
        "obsidian".to_string(),
        workspace_config::find_workspace_root().join("obsidian.zip"),
    );

    // if the filename is not in the map, return an error
    if !plugin_map.contains_key(&filename) {
        return Err("Plugin not found".into());
    }

    // Prepare paths
    let plugins_dir = workspace_config::find_plugins_dir();
    let plugins_path = plugins_dir.join(&filename);

    println!("plugins_dir: {:?}", plugins_dir);

    // decompress & move the file
    let compressed_file_name = String::from(&filename).to_owned() + &".zip".to_owned();
    let compressed_file_path = workspace_config::find_workspace_root().join(&compressed_file_name);
    let uncompressed_file_path = workspace_config::find_workspace_root().join(&filename);
    let _ = decompress_package_in_dir(&compressed_file_path);
    fs::rename(&uncompressed_file_path, &plugins_path)?;

    Ok(())
}
