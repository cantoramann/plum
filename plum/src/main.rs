// mod embeddings;
mod plugins;
use obsidian::core::obsidian_demo;

fn main() {
    obsidian_demo();

    let coord = plugins::coordinator::PlumCoordinator::new();
    coord.run("test".to_string());
}
