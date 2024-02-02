// mod embeddings;
mod coordinator;
use obsidian::core::obsidian_demo;

fn main() {
    // obsidian_demo();

    let coord = coordinator::coordinator::PlumCoordinator::new();
    coord.install("obsidian".to_string());
}
