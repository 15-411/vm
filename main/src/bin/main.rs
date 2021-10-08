use std::fs;
use std::io::{BufReader, Read};

use structopt::StructOpt;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term;

use vm::ErrorTrait;
use vm::{self, args::Config, ReturnType};


fn main() {
  let config = Config::from_args();
  let file_name = config.file_name.as_path();

  // Determine if File Exists
  let mut file = BufReader::new(
    if let Ok(file) = fs::File::open(&file_name) {
      file
    } else {
      eprintln!("\x1b[1m\x1b[91merror[F01]\x1b[0m\x1b[1m: File `{}` Not Found\x1b[0m", file_name.display());
      return;
    }
  );

  // Read File Input
  let mut file_str = String::new();
  if let Err(err) = file.read_to_string(&mut file_str) {
    eprintln!("\x1b[1m\x1b[91merror[F02]\x1b[0m\x1b[1m: Unable to Read File `{}`\x1b[0m\n{}", file_name.display(), err);
  }
  
  let mut files = SimpleFiles::new();
  let file_id = files.add(
    file_name.file_name().unwrap().to_string_lossy(), 
    &file_str,
  );

  match vm::run(&config, file_str.as_str()) {
    Ok(ReturnType::Return(val)) => println!("return {}", val),
    Ok(ReturnType::DivByZero) => println!("div-by-zero"),

    Err(err) => {
      let mut diagnostic = Diagnostic::error()
        .with_message(err.message())
        .with_code(format!("{}{}", err.tag(), err.code()));

      if let Some((label, range)) = err.label() {
        diagnostic = diagnostic.with_labels(vec![
          Label::primary(file_id, range).with_message(label)
        ]);
      }
        
      if let Some(note) = err.note() {
        diagnostic = diagnostic.with_notes(vec![note]);
      }
        
      let writer = StandardStream::stderr(ColorChoice::Always);
      term::emit(&mut writer.lock(), &term::Config::default(), &files, &diagnostic).unwrap();
    },
  }
}
