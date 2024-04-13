// Build the site
#[warn(unused_imports)]

use config::Config;
use compile::{compile_page};
use item::scan_items;

#[allow(unused)] // for cfg
pub fn build(cfg: &Config, draft: bool) {
    println!("build");

    let (mut pages, mut assets) = scan_items(String::from("./pages"));

    let (mut blogs, mut blog_assets) = scan_items(String::from("./blog"));
    
    // Preprocessing

    // Build yearly posts
    for p in &pages {
        println!("{p}");
    }
    for a in &assets {
        println!("{a}");
    }

    // Build blog

    // TODO: Nightsky log

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_imgs() {
        let (_, assets) = scan_items(String::from("tests"));
        assert_eq!(assets.len(), 0);
    }
}