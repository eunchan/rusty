// Page struct
//
// Page is consisted of Meta and Text
#![allow(unused_imports)]
use regex::Regex;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, BufRead, BufReader, Lines};
use tera::Context;
use chrono::{Local, DateTime, NaiveDate};

use crate::meta::Meta;

pub struct Page {
    pub meta: Meta,
    pub md: Option<PathBuf>,  // TODO: Markdown
    pub uri: PathBuf,
    pub text: String,
    pub html: String,
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let md_path: String = match &self.md {
            Some(f) => String::from(f.to_string_lossy()),
            None => String::from("")
        };
        write!(f, "Page({md_path})")
    }
}

impl Page {
    //= static functions ============================================
    pub fn new(path: String) -> Self {
        let mut page = Page {
            meta: Meta::new(&path),
            md: Some(PathBuf::from(path)),
            uri: PathBuf::new(),
            text: String::from(""),
            html: String::from(""),
        };

        let _ = page.load_content();

        page
    }

    // TODO: Add Meta (or fields of meta)
    #[allow(dead_code)]
    pub fn create(uri: PathBuf) -> Self {
        // Create target. In this case no markdown file is given.
        Page {
            meta: Meta::new(
                &(uri.clone()
                     .into_os_string()
                     .into_string()
                     .unwrap())),
            md: None,
            uri: uri,
            text: String::from(""),
            html: String::from(""),
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

            let _ = match lines.next().unwrap() {
                Ok(l) => l,
                Err(_) => String::from(""),
            };
            
            let meta_regex = Regex::new(r"^\-\-\-+$").unwrap();

            let mut meta_lines: Vec<String> = Vec::new();

            loop {
                let line = match lines.next().unwrap() {
                    Ok(l) => l,
                    Err(_) => break,
                };
                if meta_regex.is_match(line.as_str()) {
                    break;
                } else {
                    meta_lines.push(line);
                }
            }

            // println!("{:?}", meta_lines);

            self.meta.from_vec(meta_lines);

            // Extract rest of the lines for Markdown body
            let mut body_lines: Vec<String> = Vec::new();
            for line in lines.filter_map(|result| result.ok()) {
                body_lines.push(line);
            }
            self.text = body_lines.join("\n");
            //println!("{}", self.text);

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

    pub fn get_context(&self) -> Context {
        let mut ctx = Context::new();
        ctx.insert("title", &self.meta.title);
        ctx.insert("public", &self.meta.public);
        ctx.insert("slug", &self.meta.slug);
        ctx.insert("date", &self.meta.date.unwrap()
                                          .format("%b %-d, %-I:%M")
                                          .to_string());

        ctx
    }
}

fn read_lines(filename: &str) -> io::Result<Lines<BufReader<fs::File>>> {
    let file = fs::File::open(filename)?;
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

        // Metadata compare
        assert_eq!(item.meta.title, "Test Page".to_string());
        assert_eq!(item.meta.public, true);
        assert_eq!(item.meta.slug, "test-page".to_string());
        // DateTime
        assert_eq!(item.meta.date, Some(
            DateTime::parse_from_str("2022-12-31 00:00:00 -0800", "%Y-%m-%d %H:%M:%S %z")
                .unwrap()
                .with_timezone(&Local)));
    }

    #[test]
    fn page_context() {
        let item = Page::new(String::from("tests/page.md"));

        let _ = item.get_context();

    }
}