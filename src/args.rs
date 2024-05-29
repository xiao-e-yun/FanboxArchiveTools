use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
  /// The pattern to look for
  pub input: PathBuf,
  /// The pattern to look for
  pub output: PathBuf,
  /// run without cache
  #[clap(short, long)]
  pub force: bool,
  #[clap(short, long)]
  pub quiet: bool,
}