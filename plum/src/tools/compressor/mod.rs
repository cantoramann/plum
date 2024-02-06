use std::path::PathBuf;
use std::{
    error::Error,
    fs::{self, File},
    io,
    path::Path,
};

use zip::ZipArchive;

pub fn decompress_and_move_package(
    zip_file_path: PathBuf,
    move_loc: PathBuf,
) -> Result<(), Box<dyn Error>> {
    println!(
        "Decompressing and moving package from: to:  {} {}",
        zip_file_path.to_str().unwrap(),
        move_loc.to_str().unwrap()
    );
    let file = File::open(&zip_file_path).expect("File not found using the file path.");
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

#[cfg(test)]
mod tests {
    use super::*;

    mod decompress_valid_file {
        use super::*;

        #[test]
        fn decomrpess_using_absolute_path() {
            let zip_file_path =
                PathBuf::from("/Users/cantoraman/Documents/GitHub/plum/obsidian.zip");
            let move_loc =
                PathBuf::from("/Users/cantoraman/Documents/GitHub/plum/plugins/plugins/");

            match decompress_and_move_package(zip_file_path, move_loc) {
                Ok(_) => println!("Decompression completed successfully."),
                Err(e) => println!("Failed to decompress: {}", e),
            }
        }

        #[test]
        fn decompress_valid_file_relative_path() {
            let zip_file_path = PathBuf::from("obsidian.zip");
            let move_loc = PathBuf::from("plugins/plugins/");

            match decompress_and_move_package(zip_file_path, move_loc) {
                Ok(_) => println!("Decompression completed successfully."),
                Err(e) => println!("Failed to decompress: {}", e),
            }
        }
    }

    mod decompress_invalid_file {
        use super::*;
    }
}
