use std::env;
use std::fs;
use std::path::Path;
mod health;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: plum <command> [arguments]");
        return;
    }

    match args[1].as_str() {
        "self" => {
            if args.len() != 3 {
                println!("Usage: plum self <command>");
                return;
            }
            match args[2].as_str() {
                "update" => {
                    // Implement self update logic here
                    println!("Plum has been updated to the latest version.");
                }
                "remove" => {
                    // Removes the plum directory and the executable itself
                    let plum_dir = format!("{}/.plum", env::var("HOME").unwrap());
                    if Path::new(&plum_dir).exists() {
                        fs::remove_dir_all(plum_dir).expect("Failed to remove the plum directory");
                    }
                    println!(
                        "Plum and its directory have been removed. Please delete any projects manually."
                    );
                }
                _ => println!("Invalid command"),
            }
        }
        "new" => {
            if args.len() != 3 {
                println!("Usage: plum new <project_name>");
                return;
            }
            let project_name = &args[2];
            // Implement project creation logic here
            println!("New project '{}' created.", project_name);
        }
        "add" => {
            if args.len() != 3 {
                println!("Usage: plum add <package_name>");
                return;
            }
            let package_name = &args[2];
            // Check for .plumconfig and add package
            println!("Package '{}' added.", package_name);
        }
        "remove" => {
            if args.len() != 3 {
                println!("Usage: plum remove <package_name>");
                return;
            }
            let package_name = &args[2];
            // Check for .plumconfig and remove package
            println!("Package '{}' removed.", package_name);
        }
        _ => println!("Invalid command"),
    }
}
