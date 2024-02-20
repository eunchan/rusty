// Build the site
#[warn(unused_imports)]

use config::Config;
use item::scan_items;

#[allow(unused)] // for cfg
pub fn build(cfg: &Config, draft: bool) {
    println!("build");

    let (pages, assets) = scan_items(String::from("./pages"));
    
    for p in &pages {
        println!("{p}");
    }
    for a in &assets {
        println!("{a}");
    }
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