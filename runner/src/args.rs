use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

use structopt::StructOpt;


#[derive(Debug, Eq, PartialEq)]
pub enum Quiet {
  LinePerFile,
  Fails,
}

impl From<u64> for Quiet {
  fn from(x: u64) -> Self {
    match x {
      0 => Self::LinePerFile,
      _ => Self::Fails,
    }
  }
}

fn dir_path(path: &OsStr) -> Result<PathBuf, OsString> {
  let path = PathBuf::from(path);
  if path.is_dir() { Ok(path) } else { Err(OsString::from("Path is not a Directory")) }
}


#[derive(Debug, StructOpt)]
#[structopt(name="Compiler Test Runner", about="Test Runner Script for Compiler", no_version)]
pub struct Options {
  /// Set Level of Quiet
  #[structopt(short, parse(from_occurrences=Quiet::from))]
  pub quiet: Quiet,

  /// Number of Tests to Run in Parallel
  #[structopt(short="j", long="parallel", default_value="1")]
  pub num_parallel: u8,

  /// Path to Compiler Executable (default: ./c0c)
  #[structopt(short="b", long="bin", default_value="./c0c")]
  pub bin_path: PathBuf,

  /// Input Directory of Test Cases
  #[structopt(parse(try_from_os_str=dir_path))]
  pub inpath: PathBuf,
}
