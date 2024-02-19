// use std::{
//     error::Error,
//     fs::{self, File},
//     io,
//     path::{Path, PathBuf},
// };
// use zip::ZipArchive;

// pub fn decompress_package_in_dir(path: &PathBuf) -> Result<(), Box<dyn Error>> {
//     let move_loc = path.clone().join("..");
//     let file = File::open(&path).expect("File not found using the file path.");
//     let mut archive = ZipArchive::new(file)?;

//     println!(
//         "Decompressing the package in {}",
//         move_loc.to_str().unwrap()
//     );
//     println!("Archive len: {}", archive.len());

//     for i in 0..archive.len() {
//         let mut file = archive.by_index(i)?;
//         let outpath = match file.enclosed_name() {
//             Some(path) => path.to_owned(),
//             None => continue,
//         };

//         let outpath = match outpath.strip_prefix("zip_test/") {
//             Ok(path) => path,
//             Err(_) => &outpath,
//         };

//         let outpath = move_loc.clone().to_str().unwrap().to_owned() + outpath.to_str().unwrap();

//         if (&*file.name()).ends_with('/') {
//             fs::create_dir_all(&outpath)?;
//         } else {
//             if let Some(p) = Path::new(&outpath).parent() {
//                 if !p.exists() {
//                     fs::create_dir_all(&p)?;
//                 }
//             }
//             let mut outfile = File::create(&outpath)?;
//             io::copy(&mut file, &mut outfile)?;
//         }
//     }

//     Ok(())
// }

use std::{
    error::Error,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
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
