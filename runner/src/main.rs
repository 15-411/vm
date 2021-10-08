use std::fmt::Debug;
use std::path::PathBuf;
use std::time::Instant;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process::Command;

use structopt::StructOpt;
use rayon::prelude::*;
use rayon::iter::Either;

use vm::args::Config;
use vm::{run_wrapper as run_vm, ReturnType};


#[derive(Debug, Clone)]
pub enum TestCase {
  // Typecheck,
  Error,
  Return(i32),
  DivByZero,
  // Abort,
}

fn expected_res(entry: &PathBuf) -> Option<TestCase> {
  let file = File::open(entry).ok()?;
  let mut first_line = String::new();
  
  // Get First Line of File
  let mut buffer = BufReader::new(file);
  buffer.read_line(&mut first_line).ok()?;
  first_line = first_line.trim().trim_end_matches(";").to_string();

  if first_line == "//test error" {
    Some(TestCase::Error)
  } else if first_line == "//test div-by-zero" {
    Some(TestCase::DivByZero)
  // } else if first_line == "//test abort" {
  //   Some(TestCase::Abort)
  // } else if first_line == "//test typecheck" {
  //   Some(TestCase::Typecheck)

  } else if first_line.starts_with("//test return ") {
    if first_line.chars().nth(14)? == '~' {
      Some(TestCase::Return(-first_line[15..].parse::<i64>().ok()? as i32))
    } else {
      Some(TestCase::Return(first_line[14..].parse().ok()?))
    }
  } else {
    None
  }
}


#[derive(Debug, Eq, PartialEq)]
enum Quiet {
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
struct Options {
  /// Set Level of Quiet
  #[structopt(short, parse(from_occurrences=Quiet::from))]
  quiet: Quiet,

  /// Number of Tests to Run in Parallel
  #[structopt(short="j", long="parallel", default_value="1")]
  num_parallel: u8,

  /// Benchmark the Tests in AOT and JIT Compilation
  #[structopt(short="b")]
  bench: bool,

  /// Input Directory of Test Cases
  #[structopt(parse(try_from_os_str=dir_path))]
  inpath: PathBuf,
}


fn main() {
  let start_time = Instant::now();
  let opt: Options = Options::from_args();
  println!("{:?}", opt);
  rayon::ThreadPoolBuilder::new().num_threads(opt.num_parallel as usize)
    .stack_size(64 * 1024 * 1024).build_global().unwrap();


  // Collect and Sort Test Files
  let test_files = opt.inpath.read_dir().unwrap().collect::<Vec<_>>();
  let mut test_files = test_files.into_par_iter()
    .filter_map(|entry| entry.ok().map(|x| x.path()))
    .collect::<Vec<_>>();
  test_files.par_sort_unstable();


  // Find Expected Return
  let (test_files, ill_formed): (Vec<_>, Vec<_>) = test_files.into_par_iter()
    .filter(|path| {
      match path.extension() { None => false, Some(x) => x != "h0" }
    }).partition_map(|entry| match expected_res(&entry) {
      None => Either::Right(entry),
      Some(res) => Either::Left((entry, res)),
    });


  // Run Each Test Case
  let test_count = test_files.len();
  let (timeout_tests, failed_tests): (Vec<_>, Vec<_>) = test_files.into_par_iter()
    .filter_map(|(path, expec)| {
      // let (just_tc, header) = util::expected_header(&path);
      // let maybe_ast = compiler::run_front(&cfg);
      let ext = path.extension().unwrap().to_os_string();
      let mut compiler = Command::new("tests/lab1");
      compiler.arg(path.clone());

      let succ = match expec {
        TestCase::Error => {
          !compiler.arg("-t").output()
            .expect("Failed to run compiler")
            .status.success()
        },

        TestCase::Return(val) => {
          let success = compiler.arg("-eabs").output()
            .expect("Failed to run compiler")
            .status.success();

          if success {
            let mut abs_ext = ext.clone();
            abs_ext.push(".abs");

            run_vm(&Config::new_defaults(path.with_extension(abs_ext)))
            .map_or(false, |ret| ret == ReturnType::Return(val))

          } else {
            false
          }
        },
        
        TestCase::DivByZero => {
          let success = compiler.arg("-eabs").output()
            .expect("Failed to run compiler")
            .status.success();

          if success {
            let mut abs_ext = ext.clone();
            abs_ext.push(".abs");

            run_vm(&Config::new_defaults(path.with_extension(abs_ext)))
            .map_or(false, |ret| ret == ReturnType::DivByZero)

          } else {
            false
          }
        },
        // (true, _, Ok(_)) => Some(true),
        // (_, TestCase::Return(val), Ok(ast)) => correct_res(&cfg, ast, ReturnType::Return(val)),
        // (_, TestCase::DivByZero, Ok(ast)) => correct_res(&cfg, ast, ReturnType::DivByZero),
        // (_, TestCase::Abort, Ok(ast)) => correct_res(&cfg, ast, ReturnType::Abort),
        // (_, TestCase::Error, Err(_)) | (_, TestCase::Typecheck, Ok(_)) => Some(true),
        // _ => None
      };

      match succ {
        // Some(true) => {
        true => {
          if opt.quiet != Quiet::Fails {
            println!("\x1b[92m-- PASS: {:?} --\x1b[0m", path);
          }
          None
        },
        // Some(false) => {
        //   println!("\x1b[1m\x1b[93m-- TIME: {:?} --\x1b[0m", path);
        //   Some((path, true))
        // },
        //None => {
        false => {
          println!("\x1b[1m\x1b[91m-- FAIL: {:?} --\x1b[0m", path, );
          Some((path, false))
        },
      }
    })
    .partition_map(|(path, is_timeout)| {
      if is_timeout {
        Either::Left(path)
      } else {
        Either::Right(path)
      }
    });

  let failed_count = failed_tests.len() + timeout_tests.len();

  // Print Summary
  println!("\x1b[1m-- Summary --\x1b[0m");
  if !ill_formed.is_empty() {
    println!("-- Ill-Formed Files --");
    for file in ill_formed {
      println!("  {}", file.display())
    }
  }

  if !timeout_tests.is_empty() {
    println!("-- Timeout Tests --");
    for file in timeout_tests {
      println!("  {}", file.display())
    }
  }

  if !failed_tests.is_empty() {
    println!("-- Failed Tests --");
    for file in failed_tests {
      println!("  {}", file.display())
    }
  }

  println!("-- Elapsed Time: {:.3}s --", start_time.elapsed().as_secs_f32());
  println!("-- Passed: {} / {} --", test_count - failed_count, test_count);

  // if opt.bench {
  //   bench::bench(test_files);
  // }
}
