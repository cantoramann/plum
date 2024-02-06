use std::path::PathBuf;

mod coordinator;
mod tools;

fn main() {
    let zip_file_path = PathBuf::from("obisidan.zip");
    let move_loc = PathBuf::from("/Users/cantoraman/Documents/GitHub/plum/plugins/plugins/");

    match tools::compressor::decompress_and_move_package(zip_file_path, move_loc) {
        Ok(_) => println!("Decompression completed successfully."),
        Err(e) => println!("Failed to decompress: {}", e),
    }
}
