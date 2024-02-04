use std::{error::Error, fs::File, io, path::Path};
use zip::ZipArchive;

pub fn decompress_package(package_loc: String) -> Result<(), Box<dyn Error>> {
    let zip_file_path = Path::new("obsidian.zip");
    let zip_file = File::open(zip_file_path)?;

    let mut archive = ZipArchive::new(zip_file)?;
    let extraction_dir = Path::new("plugins/plugins");

    // Create the directory if it does not exist.
    if !extraction_dir.exists() {
        std::fs::create_dir(extraction_dir)?;
    }

    // Iterate through the files in the ZIP archive.
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_owned();

        // Create the path to the extracted file in the destination directory.
        let target_path = extraction_dir.join(file_name);

        // Create the destination directory if it does not exist.
        if let Some(parent_dir) = target_path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }

        let mut output_file = File::create(&target_path)?;

        // Read the contents of the file from the ZIP archive and write them to the destination file.
        io::copy(&mut file, &mut output_file)?;
    }

    println!("Files successfully extracted to {:?}", extraction_dir);
    Ok(())
}
