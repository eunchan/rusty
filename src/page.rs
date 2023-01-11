// Page struct
//
// Page is consisted of Meta and Text
#[warn(unused_imports)]
use regex::Regex;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{self, BufRead, BufReader, Lines};

use crate::meta::Meta;

pub struct Page {
    pub meta: Meta,
    pub md: Option<PathBuf>,  // TODO: Markdown
    pub uri: PathBuf,
    pub text: String
}

impl Page {
    //= static functions ============================================
    pub fn new(path: String) -> Self {
        let mut page = Page {
            meta: Meta::new(),
            md: Some(PathBuf::from(path)),
            uri: PathBuf::new(),
            text: String::from(""),
        };

        page.load_content();

        page
    }

    // TODO: Add Meta (or fields of meta)
    pub fn create(uri: PathBuf) -> Self {
        // Create target. In this case no markdown file is given.
        Page {
            meta: Meta::new(),
            md: None,
            uri: uri,
        text: String::from(""),
        }
    }

    //= member functions ============================================
    // load content from file (.md) and store into md
    pub fn load_content(&mut self) -> Result<(), &str> {

        let md_file = match &self.md {
            Some(f) => f,
            None => return Err("Markdown file is not configured"),
        };

        if let Ok(mut lines) = read_lines(&md_file.to_string_lossy()) {
            // Search Meta field
            // If the first line begins with `---.*` then Meta begins
            let mut in_meta = false;

            let first_line = match lines.next().unwrap() {
                Ok(l) => l,
                Err(e) => String::from(""),
            };
            
            let meta_regex = Regex::new(r"^\-\-\-+$").unwrap();

            if meta_regex.is_match(first_line.as_str()) {
                in_meta = true;
            }

            let mut meta_lines: Vec<String> = Vec::new();

            loop {
                let line = match lines.next().unwrap() {
                    Ok(l) => l,
                    Err(e) => break,
                };
                if meta_regex.is_match(line.as_str()) {
                    in_meta = false;
                    break;
                } else {
                    meta_lines.push(line);
                }
            }

            println!("{:?}", meta_lines);

            // Merge rest of the lines
            //for line in lines {
            //    if let Ok(ip) = line {
            //        dbg!("{}", ip);
            //    }
            //}
        } else {
            println!("Cannot load the file {:?}", md_file);
        }

        Ok(())
    }
}

fn read_lines(filename: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_new() {
        let item = Page::new(String::from("tests/page.md"));
        let mdpath = PathBuf::from("tests/page.md");
        assert_eq!(item.md.unwrap(), mdpath);

    }
}