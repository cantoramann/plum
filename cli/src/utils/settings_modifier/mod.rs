use super::workspace_config;
use std::fs;
use std::path::Path;
use toml::Value;
use toml_edit::{Document, InlineTable};

pub fn parse_root_workspace_members(path: &Path) -> Vec<String> {
    // Get the root and add the Cargo.toml file
    let mut root_cargo = workspace_config::find_workspace_root();
    root_cargo.push("Cargo.toml");

    println!("Root Cargo: {:?}", root_cargo);

    // Read the contents of the Cargo.toml file
    let contents = fs::read_to_string(root_cargo).expect("Failed to read Cargo.toml");

    // Parse the TOML content
    let value = contents.parse::<Value>().expect("Failed to parse TOML");

    // Navigate to the workspace members
    let members = value
        .get("workspace")
        .and_then(|ws| ws.get("members"))
        .expect("Missing workspace members");

    // Extract the array of members as Vec<String>
    let members_list: Vec<String> = members
        .as_array()
        .expect("Invalid members format")
        .iter()
        .filter_map(|member| member.as_str().map(String::from))
        .collect();

    members_list
}

pub fn add_package_to_root_workspace(
    project_root: &Path,
    mut members: Vec<String>,
    package_name: String,
) {
    // Construct the path to the root Cargo.toml
    let cargo_toml_path = project_root.join("Cargo.toml");

    // Read the contents of the root Cargo.toml file
    let contents =
        fs::read_to_string(&cargo_toml_path).expect("Failed to read Cargo.toml at project root");

    // Parse the TOML content
    let mut value: Value = contents
        .parse::<Value>()
        .expect("Failed to parse TOML content of root Cargo.toml");

    // Append the new package to the members list
    let prefixed_package_name = format!("plugins/plugins/{}", package_name);
    members.push(prefixed_package_name);

    // Update the members in the workspace configuration
    if let Some(workspace) = value.get_mut("workspace") {
        if let Some(array) = workspace.get_mut("members").and_then(|m| m.as_array_mut()) {
            *array = members.into_iter().map(Value::from).collect();
        } else {
            panic!("'members' key is not an array or is missing in the workspace configuration");
        }
    } else {
        panic!("'workspace' section is missing in the root Cargo.toml");
    }

    // Serialize the TOML object back to a string
    let updated_contents =
        toml::to_string(&value).expect("Failed to serialize updated TOML content");

    // Write the updated TOML content back to the root Cargo.toml
    fs::write(cargo_toml_path, updated_contents)
        .expect("Failed to write updated Cargo.toml at project root");

    println!(
        "Package '{}' has been added to the workspace.",
        package_name
    );
}

pub fn remove_package_from_workspace(workspace_cargo_path: &Path, package_name: &str) {
    let contents =
        fs::read_to_string(workspace_cargo_path).expect("Failed to read workspace Cargo.toml");
    let mut doc = contents.parse::<Document>().expect("Failed to parse TOML");

    // Check if the workspace members exist and remove the specified package
    if let Some(members) = doc["workspace"]["members"].as_array_mut() {
        // Construct the package path to be removed
        let package_path = format!("plugins/plugins/{}", package_name);
        // Find the index of the package path in the members array
        let index = {
            members
                .iter()
                .position(|m| m.as_str() == Some(&package_path))
        };
        // Remove the package path from the members array
        if let Some(index) = index {
            members.remove(index);
        }
    } else {
        panic!("'workspace.members' section is not an array or missing in Cargo.toml");
    }

    // Write the updated contents back to the file
    fs::write(workspace_cargo_path, doc.to_string())
        .expect("Failed to write updated workspace Cargo.toml");

    println!(
        "'{}' removed from workspace members in Cargo.toml.",
        package_name
    );
}

pub fn add_package_to_core_cargo(core_cargo_path: &Path, package_name: &str) {
    let contents = fs::read_to_string(core_cargo_path).expect("Failed to read core Cargo.toml");

    let mut doc = contents.parse::<Document>().expect("Failed to parse TOML");

    // Create an inline table to represent the dependency
    let mut dep_table = InlineTable::new();
    dep_table.get_or_insert("path", format!("../plugins/plugins/{}", package_name));

    // Ensure the "dependencies" section exists as a table, then insert the dependency
    if let Some(dep) = doc["dependencies"].as_table_mut() {
        dep.insert(package_name, toml_edit::value(dep_table));
    } else {
        eprintln!("'dependencies' section is not a table or missing in Cargo.toml");
    }

    // Write the updated contents back to the file
    fs::write(core_cargo_path, doc.to_string()).expect("Failed to write updated Cargo.toml");

    println!(
        "'{}' added to dependencies in core Cargo.toml.",
        package_name
    );
}

pub fn remove_package_from_core(core_cargo_path: &Path, package_name: &str) {
    let contents = fs::read_to_string(core_cargo_path).expect("Failed to read core Cargo.toml");
    let mut doc = contents.parse::<Document>().expect("Failed to parse TOML");

    // Remove the package from the dependencies
    if doc["dependencies"][package_name].is_table() {
        doc["dependencies"]
            .as_table_mut()
            .unwrap()
            .remove(package_name);
    } else {
        panic!(
            "Package '{}' is not found in 'dependencies' in core Cargo.toml",
            package_name
        );
    }

    // Write the updated TOML back to the core Cargo.toml
    fs::write(core_cargo_path, doc.to_string()).expect("Failed to write updated core Cargo.toml");

    println!(
        "Package '{}' has been removed from core Cargo.toml dependencies.",
        package_name
    );
}
