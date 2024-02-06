use flate2::read::GzDecoder;
use std::io::prelude::*;
use std::{
    error::Error,
    fs::{self, File},
    io,
    path::Path,
};

use zip_dir::zip_dir;

use reqwest::blocking::Client;
use zip::ZipArchive;

pub fn decompress_and_move_package(
    zip_file_path: String,
    move_loc: String,
) -> Result<(), Box<dyn Error>> {
    println!("Decompressing package {}", zip_file_path);
    println!(" to {}", move_loc);
    let file = File::open(zip_file_path).expect("File not found");
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

        let outpath = move_loc.clone() + outpath.to_str().unwrap();

        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            // error: no method named `parent` found for struct `String` in the current scope method not found in `String`
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
