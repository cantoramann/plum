use std::{
    fs,
    io::{self, BufRead, Write},
    path::Path,
};
pub fn install_mod(package_name: String) {}

// https://medium.com/@florian.bloechinger/creating-zip-files-in-rust-made-easy-with-the-zip-library-cff572906678
fn installer_precheck(package_name: String) -> bool {
    // check if the package file ./config/config.plum exists
    if !Path::new("./config/config.plum").exists() {
        // if it doesn't exist, create it
        println!("Couldn't find config file. Creating one...");
        // if ./config/ doesn't exist, create it and create the config.plum file
        if !Path::new("./config/").exists() {
            fs::create_dir("./config/").expect("Unable to create config directory");
        }

        fs::write("./config/config.plum", "init_ok").expect("Unable to create config file");
        println!("Config file created.");

        // remove the ./plugins/plugins directory if it exists
        if Path::new("./plugins/").exists() {
            fs::remove_dir_all("./plugins/").expect("Unable to delete plugins directory");
        }

        // create the ./plugins/plugins directory. Create both at once if they don't exist.
        fs::create_dir_all("./plugins/plugins/").expect("Unable to create plugins directory");
        fs::write("./plugins/plugins.config.plum", "")
            .expect("Unable to create plugins.config.plum file in plugins directory");
    } else {
        println!("Plum configuration file found. Building on top of it...")
    }

    // Check if the package file ./plugins/plugins.config.plum includes the package (package_name)
    let mut package_names = Vec::new();
    fs::read("./plugins/plugins.config.plum")
        .expect("Unable to read the configurations file")
        .lines()
        .for_each(|line| {
            package_names.push(line.unwrap().to_string());
        });

    if package_names.contains(&package_name) {
        print!("Package already installed");
        return false;
    } else {
        println!("Package not found. Attempting to install the package.");
        return true;
    }
}
