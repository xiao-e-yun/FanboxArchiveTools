use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Config {
  /// The pattern to look for
  pub input: PathBuf,
  /// The pattern to look for
  #[clap(default_value = "./archive")]
  pub output: PathBuf,
  /// run without cache
  #[clap(short, long)]
  pub force: bool,
  #[clap(short, long)]
  pub quiet: bool,
  #[clap(short, long)]
  pub verbose: bool,
}

impl Config {
  pub fn log_level(&self) -> log::LevelFilter {
    if self.verbose {
      log::LevelFilter::Debug
    } else if self.quiet {
      log::LevelFilter::Error
    } else {
      log::LevelFilter::Info
    }
  }
}