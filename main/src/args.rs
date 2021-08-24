use std::path::PathBuf;

use structopt::StructOpt;


/// Configuration options for VM
#[derive(Debug, StructOpt)]
#[structopt(name="Compiler Test Runner", about="Test Runner Script for Compiler", no_version)]
pub struct Config {
  /// Verbose Logging
  #[structopt(short="v")]
  pub verbose: bool,

  /// Enable Strict SSA Mode
  #[structopt(long="ssa")]
  pub ssa: bool,

  /// Input Directory of Test Cases
  #[structopt(name = "FILE", parse(from_os_str))]
  pub file_name: PathBuf,
}
