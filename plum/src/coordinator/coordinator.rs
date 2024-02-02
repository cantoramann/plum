use std::{
    fs,
    io::{self, BufRead, Write},
    path::Path,
};

pub(crate) struct PlumCoordinator {
    status: String,
}

impl PlumCoordinator {
    pub fn new() -> Self {
        // check if the coordinator file ./config/config.plum exists
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
            println!("Configuration found.")
        }

        return PlumCoordinator {
            status: "ok".to_string(),
        };
    }

    pub fn install(&self, package_name: String) {
        let package_name = package_name.to_lowercase().replace(" ", "_");

        let mut package_names = Vec::new();
        fs::read("./plugins/plugins.config.plum")
            .expect("Unable to read the configurations file")
            .lines()
            .for_each(|line| {
                package_names.push(line.unwrap().to_string());
            });

        if package_names.contains(&package_name) {
            print!("Package already installed");
        } else {
            println!("Package not found. Attempting to install the package.");
            self.installer(package_name);
        }
    }

    fn installer(&self, package_name: String) {
        let full_path = "https://plum-registry.sh/".to_string() + &package_name + ".zip";
        println!("Pulling package: {}", full_path);

        // Pull the package -- EXPERIMENTAL --
        let response = reqwest::blocking::get(&full_path).expect("Unable to get package");
        let mut file = fs::File::create("./plugins/plugins/".to_string() + &package_name)
            .expect("Unable to create file");
        std::io::copy(&mut response.bytes().unwrap().as_ref(), &mut file)
            .expect("Unable to copy file");

        // Modify the root Cargo.toml file to include the package (naive approach)
        let _ = self.cargo_config_modifier(package_name);
    }

    fn cargo_config_modifier(&self, package_name: String) -> io::Result<()> {
        let cargo_toml_path = "Cargo.toml"; // Adjust the path as necessary for your environment
        let mut contents = fs::read_to_string("Cargo.toml".to_string())?;

        if let Some(pos) = contents.rfind('\"') {
            contents.truncate(pos + 1);

            let formatted_package_name = format!(", \"plugins/plugins/{}\"]", package_name);
            contents.push_str(&formatted_package_name);
            let mut file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(cargo_toml_path)?;

            file.write_all(contents.as_bytes())?;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not find the end of members list in Cargo.toml",
            ));
        }
        Ok(())
    }
}
