use ansi_term::Colour::Red;
use std::path::PathBuf;

use super::{installation, settings_modifier, workspace_config};

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

    // check if the package is already installed
    let members = settings_modifier::parse_root_workspace_members(&project_root);
    let prefixed_package_name = format!("plugins/plugins/{}", package_name);
    if members.contains(&prefixed_package_name) {
        println!(
            "{}",
            Red.paint(format!("Package '{}' is already installed.", package_name))
        );

        return;
    }

    // install the package
    let _res = installation::add_plugin(package_path.to_str().unwrap(), package_name.clone());

    // write to root Cargo.toml
    settings_modifier::add_package_to_root_workspace(&project_root, members, package_name.clone());
    settings_modifier::add_package_to_core_cargo(
        &project_root.join("core").join("Cargo.toml"),
        &package_name,
    );

    println!("Package '{}' added.", package_name);
}
