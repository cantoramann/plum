use reqwest::blocking::Client;
use std::{error::Error, fs::File, io};

pub fn clone_from_remote(path: String, filename: String) -> Result<(), Box<dyn Error>> {
    let mut resp = Client::new().get(&path).send()?;
    let mut out = File::create(filename)?;
    io::copy(&mut resp, &mut out)?;
    Ok(())
}
