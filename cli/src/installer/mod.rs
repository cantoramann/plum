use crate::checks;
use reqwest::blocking::Client;
use std::{
    error::Error,
    fs::File,
    io::{self, Write},
};

pub fn add_plugin(path: String, filename: String) -> Result<(), Box<dyn Error>> {
    let workspace_root = checks::find_workspace_root();

    let mut resp = Client::new().get(&path).send()?;
    let mut out = File::create(filename)?;
    io::copy(&mut resp, &mut out)?;
    Ok(())
}

pub fn tidy_plugins() {
    let workspace_root = checks::find_workspace_root();
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
