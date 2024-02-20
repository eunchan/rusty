// Common library
#[warn(unused_imports)]

use walkdir::WalkDir;

use page::Page;
use asset::Asset;

// scan directory and store into a data structure (or cache?)
// pub fn scan_assets() -> <Vec<Page>, Vec<Asset>>

pub fn scan_items(dir: String) -> (Vec<Page>, Vec<Asset>) {
    let mut pages: Vec<Page> = Vec::new();
    let mut assets: Vec<Asset> = Vec::new();

    // Search all under dir
    for entry in WalkDir::new(dir.as_str())
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            // filter only file not dir
            .filter(|x| x.file_type().is_file()) {
        
        let path = entry.path();
        let name = entry.file_name().to_string_lossy();
        let ext = name.split(".").last().unwrap();

        match ext {
            "md" => {
                let mut p = Page::new(
                    String::from(entry.path().to_string_lossy()
                ));
                let _ = p.load_content();
                pages.push(p);
            },
            "jpg" | "png" | "svg" => assets.push(
                Asset::new(path.to_path_buf())),
            "zip" | "gz" | "dat" | "tar" => assets.push(
                Asset::new(path.to_path_buf())),
            _ => println!("Unexpected file: {}", path.display()),
        }
    }
    
    (pages, assets)
}

