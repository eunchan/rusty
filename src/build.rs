// Build the site
#[warn(unused_imports)]

use walkdir::WalkDir;

use super::config::Config;

pub fn build(cfg: &Config, draft: bool) {
    println!("build");

    // lists for each category
    let mut doc = Vec::new();
    let mut img = Vec::new();
    let mut file = Vec::new();

    let dir: String = ".".to_owned();

    // Search all under pages/
    let pages = dir + "/pages";

    for entry in WalkDir::new(pages.clone().as_str())
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            // filter only file not dir
            .filter(|x| x.file_type().is_file()) {
        
        let path = entry.path();
        let name = entry.file_name().to_string_lossy();
        let ext = name.split(".").last().unwrap();

        match ext {
            "md" => doc.push(path.to_owned()),
            "jpg" | "png" | "svg" => img.push(path.to_owned()),
            "zip" | "gz" | "dat" | "tar" => file.push(path.to_owned()),
            _ => println!("Unexpected file: {}", path.display()),
        }
    }
    
    println!("Markdown docs: {}", doc.len());
}
