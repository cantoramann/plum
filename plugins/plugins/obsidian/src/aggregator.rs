use std::{fs, path::Path};

pub(crate) struct Aggregator {
    main_path: String,
    contents: Vec<String>,
    filenames: Vec<String>,
}

impl Aggregator {
    pub fn new() -> Self {
        return Aggregator {
            main_path: "/Users/cantoraman/Library/Mobile Documents/iCloud~md~obsidian/Documents"
                .to_string(),
            contents: Vec::new(),
            filenames: Vec::new(),
        };
    }

    pub fn aggregate(&mut self) {
        // implement recursive search into the path to get all .md files
        let root_path =
            Path::new("/Users/cantoraman/Library/Mobile Documents/iCloud~md~obsidian/Documents");

        if let Err(e) = self.recurse_notes(root_path) {
            eprintln!("An error occurred: {}", e);
        }
    }

    fn recurse_notes(&mut self, path: &Path) -> Result<(), std::io::Error> {
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    self.recurse_notes(&path)?;
                } else {
                    if let Some(ext) = path.extension() {
                        if ext == "md" {
                            let filename = path.file_name().unwrap().to_str().unwrap().to_string();
                            let content = fs::read_to_string(&path)?;

                            self.contents.push(content);
                            self.filenames.push(filename);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get_aggregator_data(&self) -> Box<Vec<Vec<String>>> {
        // create a 2D vector of filenames and contents
        let mut data: Vec<Vec<String>> = Vec::new();

        for i in 0..self.contents.len() {
            let mut row: Vec<String> = Vec::new();
            row.push(self.filenames[i].clone());
            row.push(self.contents[i].clone());

            data.push(row);
        }

        return Box::new(data);
    }
}
