use vm::{self, args, ReturnType};

fn main() {
  let (file_name, _) = vm::args::parse_args();
  match vm::run(file_name) {
    None => eprintln!("Parse Error"),
    Some(ReturnType::Return(val)) => println!("{}", val),
    Some(ReturnType::DivByZero) => println!("div-by-zero"),
    _ => unreachable!()
  }
}
