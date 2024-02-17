/*
* CHECK FOR UNNEEDEED PARTS
*/
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Result, Seek, SeekFrom, Write};
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

enum BorrFileOrBorrBuffer<'a> {
    File(&'a File),
    Buffer(BufReader<&'a File>),
}

fn parse_cargo_members<'a>(
    reader_opts: BorrFileOrBorrBuffer<'a>,
    pattern: &str,
) -> Result<(Vec<String>, usize)> {
    let mut reader = match reader_opts {
        BorrFileOrBorrBuffer::File(file) => BufReader::new(file),
        BorrFileOrBorrBuffer::Buffer(buffer) => buffer,
    };

    // Convert the pattern to bytes for comparison
    let pattern_bytes = pattern.as_bytes();

    let mut buffer = Vec::new();
    // Read the entire content into a buffer
    reader.read_to_end(&mut buffer)?;

    // Search for the pattern, assuming it includes the pattern
    if let Some(start_index) = buffer
        .windows(pattern.as_bytes().len())
        .position(|window| window == pattern.as_bytes())
    {
        let members_start = start_index + pattern.len();

        // Assuming the array ends with ']', find the end of the array
        let members_end = buffer[members_start..]
            .iter()
            .position(|&b| b == b']')
            .unwrap_or_else(|| buffer.len() - members_start)
            + members_start;
        let members_str = str::from_utf8(&buffer[members_start..members_end])
            .expect("Failed to convert bytes to string");

        let members: Vec<String> = members_str
            .trim_matches(|p: char| p == '[' || p == ']')
            .split(',')
            .filter_map(|s| {
                let member = s.trim().trim_matches('"');
                if !member.is_empty() {
                    Some(member.to_string())
                } else {
                    None
                }
            })
            .collect();

        return Ok((members, members_start));
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "Pattern not found"))
}

fn write_array_from_start(file: &mut File, start: usize, members: &Vec<String>) -> io::Result<()> {
    let mut writer = BufWriter::new(file);

    // Prepare to write from the specified start position
    writer.seek(SeekFrom::Start(start as u64))?;

    // Stringify and write the updated members list
    let members_string = members
        .into_iter()
        .map(|name| format!("\"{}\"", name))
        .collect::<Vec<String>>()
        .join(", ");
    let array_string = format!("[{}]", members_string);

    writer.write_all(array_string.as_bytes())?;
    writer.flush()?; // Ensure all data is written

    Ok(())
}

pub fn accept_update_cargo(args: CargoFileModifierArgs) -> io::Result<()> {
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

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .unwrap();

    match args.operation {
        CargoOperationType::AddMemberAsPackage { package_name } => {
            let read_res = parse_cargo_members(BorrFileOrBorrBuffer::File(&file), "members");
            if let Ok((members, start)) = read_res {
                if members.contains(&package_name.to_string()) {
                    println!("Package already exists in the members list. Aborting.");
                    return Ok(());
                }

                let _write_res = write_array_from_start(&mut file, start, &members);
            }
        }
    }

    Ok(())
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_cargo_members() {
        let file = File::open("Cargo.toml").unwrap();
        let members = parse_cargo_members(BorrFileOrBorrBuffer::File(&file), "members");
        assert!(members.is_ok());
    }

    #[test]
    fn test_write_array_from_start() {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("Cargo.toml")
            .unwrap();
        let members = vec!["test".to_string(), "test2".to_string()];
        let write_res = write_array_from_start(&mut file, 0, &members);
        assert!(write_res.is_ok());
    }
}
