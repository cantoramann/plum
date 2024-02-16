use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Result, Seek, SeekFrom, Write};
use std::path::Path;
use std::str;

struct AddMemberAsPackage<'a> {
    package_name: &'a str,
}

enum CargoOperationType<'a> {
    AddMemberAsPackage { package_name: &'a str },
}

struct CargoFileModifierArgs<'a> {
    filepath: &'a str,
    operation: CargoOperationType<'a>,
}

enum FileOrBuffer<'a> {
    File(&'a File),
    Buffer(BufReader<&'a File>),
}

fn parse_cargo_members<'a>(
    reader_opts: FileOrBuffer<'a>,
    pattern: &str,
) -> Result<(Vec<String>, usize, usize)> {
    let mut reader = match reader_opts {
        FileOrBuffer::File(file) => BufReader::new(file),
        FileOrBuffer::Buffer(buffer) => buffer,
    };

    // Convert the pattern to bytes for comparison
    let pattern_bytes = pattern.as_bytes();
    let mut buffer = Vec::new();

    // Read the entire content into a buffer
    reader.read_to_end(&mut buffer)?;

    // Search for the pattern in the buffer
    if let Some(start_index) = buffer
        .windows(pattern_bytes.len())
        .position(|window| window == pattern_bytes)
    {
        // Find the start of the members list after '='
        if let Some(start_of_members) = buffer[start_index..].iter().position(|&b| b == b'=') {
            let members_start = start_index + start_of_members + 2; // Skip '=' and the space
                                                                    // Find the end of the members list
            if let Some(end_of_members) = buffer[members_start..].iter().position(|&b| b == b']') {
                let members_end = members_start + end_of_members; // Include ']' in the output
                let members_str = str::from_utf8(&buffer[members_start..=members_end])
                    .expect("Failed to convert bytes to string");

                let members: Vec<String> = members_str
                    .trim_matches(|p: char| p == '[' || p == ']')
                    .split(',')
                    .map(|s| s.trim().trim_matches('"').to_string())
                    .collect();

                return Ok((members, members_start, members_end));
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "Pattern not found"))
}

fn add_cargo_member(writer_opts: FileOrBuffer, string: &str) {}

pub fn accept_update_cargo(args: CargoFileModifierArgs) {
    // Check if the file exists

    // path is the project root
    let mut path = String::from(args.filepath);
    if args.filepath.is_empty() {
        path.push_str("Cargo.toml");
    } else {
        path.push_str(args.filepath);
    }

    let path = Path::new(&path);
    // path must exist. path must be a file
    if !path.exists() {
        if args.filepath.is_empty() {
            panic!("Could not locate a Cargo.toml file in the project root. Check configuration or make an issue. Aborting.");
        }
        panic!("Could not locate the specified Cargo file in the arguments. Aborting.");
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .unwrap();

    match args.operation {
        CargoOperationType::AddMemberAsPackage { package_name } => {
            let parse_res = parse_cargo_members(FileOrBuffer::File(&file), "members");
            if let Ok((members, start, end)) = parse_res {
                if members.contains(&package_name.to_string()) {
                    println!("Package already exists in the members list. Aborting.");
                    return;
                }
                let mut members = members;
                members.push(package_name.to_string());
                let mut file = OpenOptions::new()
                    .write(true)
                    .open(path)
                    .expect("Failed to open the file for writing");
                file.seek(SeekFrom::Start(start as u64)).unwrap();
                file.write_all(b"members = [").unwrap();
                for member in members {
                    file.write_all(format!("\"{}\", ", member).as_bytes())
                        .unwrap();
                }
                file.write_all(b"]").unwrap();
            }
        }
    }
}
