use crate::configuration;
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
    // new
    let plugins_path = configuration::find_workspace_root()
        .join("plugins")
        .join("plugins");

    // Ensure the plugins directory exists.
    fs::create_dir_all(&plugins_path)?;

    // Combine the directory path with the filename to get the target path.
    let file_path: std::path::PathBuf = plugins_path.join(&filename);

    // new command to get into plugins/plugins directory
    let _ = Command::new("curl")
        .arg(remote_path)
        .current_dir(plugins_path)
        .output()
        .unwrap();

    Ok(())
}

pub fn tidy_plugins() {
    let workspace_root = configuration::find_workspace_root();
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
