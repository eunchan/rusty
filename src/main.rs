use tera::Tera;

// rusty Main function
mod config;
mod meta;
mod page;
mod template;

use config::Config;
use template::Template;

pub fn main() {
  // TODO: Parse arg
  
  // Load config
  let cfg = Config::read_file("config.toml");
  println!("{:?}", cfg);
}
