mod coordinator;
mod tools;
use obsidian::core::obsidian_demo;

fn main() {
    // obsidian_demo();

    // let coord = coordinator::coordinator::PlumCoordinator::new();
    // coord.install("obsidian".to_string());

    // Testing the installer
    let _ = tools::compressor::decompress_package(String::from("obsidian.zip"));
}
