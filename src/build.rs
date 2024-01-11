// Build the site
#[warn(unused_imports)]

use crate::config::Config;
use crate::item::scan_items;

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
