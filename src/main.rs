#![allow(unused_imports)]
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

// rusty Main function

use config::Config;
use template::Template;
use build::build;

#[derive(Parser, Debug)]
#[command(name = "Rusty")]
#[command(author = "Eli Kim <me@eunchan.kim>")]
#[command(version = "0.1")]
#[command(about = "Personal Website Generator", long_about = None)]
struct Cli {
  // config.toml path
  #[arg(short, long, value_name = "FILE")]
  config: Option<PathBuf>,

  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
  // Build Website
  Build {
    // build pages with Draft
    #[arg(short, long, default_value = "true")]
    draft: bool,
  },
  Deploy,
  Watch {
    // Preview or release
    #[arg(short, long, default_value = "true")]
    draft: bool,
  },
}

fn main() {
  // TODO: Parse arg  
  let arg = Cli::parse();

  // Config
  let cfg_path: &Path;

  if let Some(config) = arg.config.as_deref() {
    cfg_path = config;
  } else {
    cfg_path = Path::new("config.toml");
  }  

  // Load config
  let cfg = Config::read_file(&String::from(cfg_path.to_string_lossy()));
  println!("{:?}", cfg);

  // Subcommands
  match arg.command {
    Some(Commands::Build { draft }) => {
      build(&cfg, draft);
    }
    // Some(Commands::Deploy)
    // Some(Commands::Watch { draft })
    _ => {
      println!("Not implemented");
    }
  }
}
