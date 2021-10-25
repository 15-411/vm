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

  /// Enable Strict SSA Mode
  #[structopt(long="timeout")]
  pub timeout: Option<u64>,

  /// Input Directory of Test Cases
  #[structopt(name = "FILE", parse(from_os_str))]
  pub file_name: PathBuf,
}

impl Config {
  pub fn new_defaults(file_name: PathBuf) -> Self {
    Self { file_name, ssa: false, verbose: false, timeout: None }
  }

  pub fn new_timeout(file_name: PathBuf, timeout: u64) -> Self {
    Self { file_name, ssa: false, verbose: false, timeout: Some(timeout) }
  }
}
