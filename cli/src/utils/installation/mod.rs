use crate::utils::compression::decompress_package_in_dir;
use crate::utils::workspace_config;
use std::error::Error;
use std::fs;

/*

* Installs the plugin from the given path to the plugins directory.
*/
pub fn add_plugin(remote_path: &str, filename: String) -> Result<(), Box<dyn Error>> {
    println!("Installing plugin from {}", remote_path);
    println!("Installing plugin {}", filename);

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
    let plugins_path = plugins_dir.join(&filename);

    // // Ensure the plugins directory exists.
    // fs::create_dir_all(&plugins_dir)?;

    // // Combine the directory path with the filename to get the target path.
    // let file_path: std::path::PathBuf = plugins_dir.join(&filename);

    // new command to get into plugins/plugins directory
    println!("plugins_dir: {:?}", plugins_dir);

    let compressed_file_name = String::from(&filename).to_owned() + &".zip".to_owned();
    let compressed_file_path = workspace_config::find_workspace_root().join(&compressed_file_name);
    let uncompressed_file_path = workspace_config::find_workspace_root().join(&filename);
    let _ = decompress_package_in_dir(&compressed_file_path);

    // move the file to the plugins directory
    fs::rename(&uncompressed_file_path, &plugins_path)?;

    // delete __MACOSX directory
    // todo :: after remote path is implemented

    Ok(())
}
