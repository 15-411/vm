mod args;

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::PathBuf;
use std::time::Instant;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process::Command;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

use itertools::Itertools;
use rayon::prelude::*;
use rayon::iter::Either;
use indicatif::{ProgressBar, ProgressStyle};
use structopt::StructOpt;

use vm::args::Config;
use vm::{run_wrapper as run_vm, ReturnType};
use crate::args::{Options, Quiet};


#[derive(Debug, Clone)]
pub enum TestCase {
  Typecheck,
  Error,
  Return(i32),
  DivByZero,
  // Abort,
}

impl Display for TestCase {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    match self {
      TestCase::Error => write!(f, "error"),
      TestCase::Return(val) => write!(f, "return {}", val),
      TestCase::DivByZero => write!(f, "div-by-zero"),
      TestCase::Typecheck => write!(f, "typecheck"),
    }
  }
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
  } else if first_line == "//test typecheck" {
    Some(TestCase::Typecheck)
  // } else if first_line == "//test abort" {
  //   Some(TestCase::Abort)


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

enum Passing {
  Starting(usize, String),
  Ending(usize)
}

fn exec(mut compiler: Command, path: &PathBuf, expected_ret: ReturnType) -> Option<bool> {
  let ext = path.extension().unwrap().to_os_string();

  let exec_success = compiler.arg("-eabs").output()
    .expect("Failed to run compiler")
    .status.success();

  if exec_success {
    let mut abs_ext = ext.clone();
    abs_ext.push(".abs");
    let new_path = path.with_extension(abs_ext);

    let res = match run_vm(&Config::new_timeout(new_path.clone(), 6)) {
      Err(_) => Some(true),
      Ok(ReturnType::Timeout) => Some(false),
      Ok(ret) if ret == expected_ret => None,
      _ => Some(true),
    };

    let _ = std::fs::remove_file(new_path);
    res

  } else {
    Some(true)
  }
}


fn main() {
  let start_time = Instant::now();
  let opt: Options = Options::from_args();
  rayon::ThreadPoolBuilder::new().num_threads(opt.num_parallel as usize)
    .stack_size(64 * 1024 * 1024)
    .build_global().unwrap();

  // Collect and Sort Test Files
  let test_files = opt.inpath.read_dir().unwrap().collect::<Vec<_>>();
  let mut test_files = test_files.into_par_iter()
    .filter_map(|entry| entry.ok().map(|x| x.path()))
    .collect::<Vec<_>>();
  test_files.par_sort_unstable();
  let test_count = test_files.len() as u64;


  // Find Expected Return
  let (test_files, ill_formed): (Vec<_>, Vec<_>) = test_files.into_par_iter()
    .filter(|path| {
      match path.extension() { None => false, Some(x) => x != "h0" }
    }).partition_map(|entry| match expected_res(&entry) {
      None => Either::Right(entry),
      Some(res) => Either::Left((entry, res)),
    });


  // Set Up Progress Bar
  let pb = ProgressBar::new(test_files.len() as u64);
  pb.set_style(ProgressStyle::default_bar()
    .template("({elapsed_precise}) {pos}/{len} [{bar:60.blue}] {wide_msg}")
    .progress_chars("##-"));

  // Set Up Thread to Collect and Modify Progress Bar
  let (tx, rx): (Sender<Passing>, Receiver<Passing>) = mpsc::channel();
  let child = thread::spawn(move || {
    let mut running_tests = HashMap::new();

    while pb.position() < test_count {
      match rx.recv() {
        Ok(Passing::Starting(test_idx, test_case)) => {
          if running_tests.len() < 4 {
            running_tests.insert(test_idx, test_case);
          }
          
          pb.inc(1);
          pb.set_message(running_tests.values().format(", ").to_string());
        },

        Ok(Passing::Ending(test_idx)) => {
          running_tests.remove(&test_idx);
        },

        Err(_) => break,
      }
    }

    pb.finish_with_message("Done!");
  });


  // Run Each Test Case
  let test_count = test_files.len();
  let (timeout_tests, failed_tests): (Vec<_>, Vec<_>) = test_files.into_par_iter()
    .enumerate()
    .map_with(tx, |tx, (idx, (path, expec))| {
      let path_str = path.as_path().file_name().unwrap().to_str().unwrap().to_string();
      let _ = tx.send(Passing::Starting(idx, path_str));

      // std::thread::sleep(std::time::Duration::from_secs(1));
      // Print Basic Info
      // println!("Running `{}` expecting {}", path.display(), expec);

      let mut compiler = Command::new(opt.bin_path.as_os_str().to_str().unwrap());
      compiler.arg(path.clone());

      let succ = match expec {
        TestCase::Error => {
          let output = compiler.arg("-t").output()
            .expect("Failed to run compiler");

          // println!("{} {}", String::from_utf8(output.stdout).unwrap(), String::from_utf8(output.stderr).unwrap());
          if !output.status.success() { None } else { Some(true) }
        },

        TestCase::Return(val) => exec(compiler, &path, ReturnType::Return(val)),
        TestCase::DivByZero => exec(compiler, &path, ReturnType::DivByZero),

        TestCase::Typecheck => None,
      };

      let _ = tx.send(Passing::Ending(idx));

      // let res = match succ {
      //   // Some(true) => {
      //   true => {
      //     // if opt.quiet != Quiet::Fails {
      //       // pb.println("test");
      //       // println!("\x1b[92m-- PASS: {:?} --\x1b[0m", path);
      //     // }
      //     None
      //   },
      //   // Some(false) => {
      //   //   println!("\x1b[1m\x1b[93m-- TIME: {:?} --\x1b[0m", path);
      //   //   Some((path, true))
      //   // },
      //   //None => {
      //   false => {
      //     // println!("\x1b[1m\x1b[91m-- FAIL: {:?} --\x1b[0m", path, );
      //     Some((path, false))
      //   },
      // };


      succ.map(|val| (path, val))
    })

    .filter_map(|x| x)
    .partition_map(|(path, is_fail)| {
      if is_fail {
        Either::Right(path)
      } else {
        Either::Left(path)
      }
    });

  let failed_count = failed_tests.len();
  let timeout_count = timeout_tests.len();

  // Join Handler
  let _ = child.join();

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

  println!("-- Elapsed Time: {:.2}s --", start_time.elapsed().as_secs_f32());
  println!("-- Passed:  {} / {} --", test_count - failed_count - timeout_count, test_count);
  println!("-- Failed:  {} / {} --", failed_count, test_count);
  println!("-- Timeout: {} / {} --", timeout_count, test_count);

  // TODO: Add Autograder Calculation
}
