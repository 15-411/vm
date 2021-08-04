mod args;
mod asm;
mod exec;
mod ops;
mod parser;

use std::fs::File;
use std::io::{BufReader, Read};

use exec::{ProgContext, ReturnType};


fn main() {
  let (file_name, _) = args::parse_args();
  let mut file = BufReader::new(
    File::open(&file_name).unwrap_or_else(|_| panic!("File {} not found", file_name))
  );

  // Read File Input
  let mut file_str = String::new();
  if let Err(err) = file.read_to_string(&mut file_str) {
    eprintln!("Unable to Read File {}: {}", file_name, err);
  }

  let parse_res = parser::parse(file_str);
  let abs_asm = match parse_res {
    Ok(res) => res,
    Err(e) => {
      eprintln!("Parse Error: {}", e);
      return; // Parse failed!
    }
  };

  match ProgContext::run(abs_asm) {
    ReturnType::Return(val) => println!("{}", val),
    ReturnType::DivByZero => println!("div-by-zero"),
    _ => unreachable!()
  }
}
