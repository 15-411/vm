use structopt::StructOpt;
use vm::{self, args::Config, ReturnType};

fn main() {
  let config = Config::from_args();
  match vm::run(config.file_name.as_path()) {
    None => eprintln!("Parse Error"),
    Some(ReturnType::Return(val)) => println!("{}", val),
    Some(ReturnType::DivByZero) => println!("div-by-zero"),
    _ => unreachable!()
  }
}
