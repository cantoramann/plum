use ansi_term::Colour::Red;
use std::path::{Path, PathBuf};

use super::{installation, workspace_config};

pub fn install_package_handler(package_name: String) {
    // check if the workspace is correctly configured
    let project_root = workspace_config::find_workspace_root();
    let package_path: PathBuf = project_root.join(format!("{}.zip", package_name));
    if !package_path.exists() {
        println!(
            "{}",
            Red.paint(format!(
                "Looks like the remote path for '{}' is invalid.",
                package_name
            ))
        );

        return;
    }
    let _res = installation::add_plugin(package_path.to_str().unwrap(), package_name.clone());
    println!("Package '{}' added.", package_name);
}
