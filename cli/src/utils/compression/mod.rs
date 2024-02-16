use std::{
    error::Error,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};
use zip::ZipArchive;

pub fn decompress_package_in_dir(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let move_loc = path.clone().join("..");
    let file = File::open(&path).expect("File not found using the file path.");
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        let outpath = match outpath.strip_prefix("zip_test/") {
            Ok(path) => path,
            Err(_) => &outpath,
        };

        let outpath = move_loc.clone().to_str().unwrap().to_owned() + outpath.to_str().unwrap();

        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = Path::new(&outpath).parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}
