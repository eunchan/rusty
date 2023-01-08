/* Config struct and parser
 */
#[warn(unused_imports)]

use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    title: String,
    author: String,
    target_dir: String,
    fileserver: FileServer,
    watch: Watch,
}

// Fileserver config if used
#[derive(Deserialize, Debug)]
struct FileServer {
    is_fileserver: bool,
    domain: String,
    fs_type: FileServerType,
}

#[derive(Deserialize, Debug)]
struct Watch {
    domain: String,
    port: u16,
}

// Other possible values are lowercase, UPPERCASE, PascalCase, camelCase,
// snake_case, SCREAMING_SNAKE_CASE, kebab-case, and SCREAMING-KEBAB-CASE
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FileServerType {
    Gcs,
    Github,
    S3,
}

impl Config {
    // Read .toml file and return config struct
    pub fn read_file(path: &str) -> Self {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Could not read file `{}`", path);
                exit(1);
            }
        };

        let cfg: Config = match toml::from_str(&content) {
            Ok(c) => c,
            Err(msg) => {
                eprintln!("Unable to load data from `{}`:", path);
                eprintln!("{msg}");
                exit(1);
            }
        };

        cfg
    }
}
