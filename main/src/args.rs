use std::env;


/// Configuration options for this compiler run.
pub struct Config {
  pub verbose: bool,
  pub ssa: bool,
}

impl Config {
  /// Set your defaults here!
  fn default() -> Self {
    Config {
      verbose: false,   // Get extra output from the compiler
      ssa: true,        // SSA Checks Only
    }
  }
}

/// Parses command line input into a configuration. Panics on invalid args.
pub fn parse_args() -> (String, Config) {
  let args: Vec<String> = env::args().collect();
  let mut config = Config::default();
  let mut file_name = None;
  let mut index = 1;

  while index < args.len() {
    match args[index].as_str() {
      "-v" | "--verbose" => config.verbose = true,
      "-s" | "--ssa" => config.verbose = true,
      file => {
        if let Some('-') = file.chars().next() {
        } else {
          file_name = Some(file.to_string())
        }
      }
    };
    index += 1;
  }

  (file_name.expect("Expect File Input"), config)
}
