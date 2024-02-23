use std::{
    error::Error,
    fs::{self, File},
    io::{self},
    path::Path,
};
use zip::ZipArchive;

pub fn decompress_package_in_dir(path: &Path) -> Result<(), Box<dyn Error>> {
    // Ensure `move_loc` correctly references the directory you intend to use
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = file.enclosed_name().ok_or("Invalid file name in archive")?;

        // Use `PathBuf` for path manipulation
        let mut full_path = path.parent().unwrap_or_else(|| Path::new("")).to_path_buf();
        full_path.push(outpath);

        if file.name().ends_with('/') {
            fs::create_dir_all(&full_path)?;
        } else {
            if let Some(parent) = full_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            let mut outfile = File::create(&full_path)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}
