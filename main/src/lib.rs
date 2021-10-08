// Macro for Creating a Set Quickly
/// Similar to the vec![] macro
#[macro_export]
macro_rules! set {
  () => {fxhash::FxHashSet::default()};  // Match zero items / empty
  ( $( $x:expr ),* ) => {  // Match one or more comma delimited items
    {
      let mut temp_set = fxhash::FxHashSet::default();  // Create a mutable HashSet
      $(
        temp_set.insert($x); // Insert each item matched into the HashSet
      )*
      temp_set // Return the populated HashSet
    }
  };
}

mod error;
pub mod args;
mod asm;
mod exec;
mod ops;
mod parser;
mod analysis;

use std::fs::File;
use std::io::{BufReader, Read};


use analysis::sem_analysis;
use args::Config;
use exec::ProgContext;
pub use exec::ReturnType;
pub use error::{Error, ErrorTrait};


pub fn run(config: &Config, file_str: &str) -> Result<ReturnType, Error> {
  let parse_res = parser::parse(file_str);
  let abs = parse_res.map_err(|e| Error::ParseError(e))?;  
  sem_analysis(&abs, config.ssa).map_err(|e| Error::SemError(e))?;

  // TODO: Verbose
  // for (_, func) in abs_asm.iter() {
  //   println!("{}", func);
  // }

  // TODO: Any errors here?
  Ok(ProgContext::run(abs))
}

pub fn run_wrapper(config: &Config) -> Result<ReturnType, Error> {
  let mut file = BufReader::new(
    File::open(&config.file_name).unwrap_or_else(|_| panic!("File {} not found", config.file_name.display()))
  );

  // Read File Input
  let mut file_str = String::new();
  if let Err(err) = file.read_to_string(&mut file_str) {
    eprintln!("Unable to Read File {}: {}", config.file_name.display(), err);
  }

  run(config, file_str.as_str())
}
