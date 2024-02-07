use std::env;

mod coordinator;
mod tools;

fn main() {
    let path: std::path::PathBuf = env::current_dir().unwrap();
    println!("The current directory is: {:?}", path);

    match tools::compressor::decompress_and_move_package(
        path.clone().join("obsidian.zip"),
        path.clone().join("plugins/plugins/"),
    ) {
        Ok(_) => println!("Decompression completed successfully."),
        Err(e) => println!("Failed to decompress: {}", e),
    }
}
