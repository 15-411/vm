use std::path::PathBuf;
use std::time::Instant;
use std::process::{Command, Stdio};

use itertools::Itertools;
use prettytable::{Table, row, cell};
use prettytable::format::{FormatBuilder, LinePosition, LineSeparator};

use compiler::Config;
use compiler::jit::{ProgContext, ReturnType};
use compiler::emit::emit_x86;

use super::TestCase;
use super::util::expected_header;


const NUM_RUNS: u128 = 25;

pub fn bench(test_files: Vec<(PathBuf, TestCase)>) {
  let return_tests = test_files.into_iter()
    .filter_map(|(path, ret_type)|
      if let TestCase::Return(ret_val) = ret_type { Some((path, ret_val)) } else { None }
    ).collect::<Vec<_>>();

  let mut bench_res: Vec<(u128, u128, u128)> = vec![];
  println!("\n-- Running Benchmarking --");

  for (path, ret_val) in return_tests.iter() {
    let mut in_jit_run = 0;
    let mut in_aot_compile = 0;
    let mut in_aot_run = 0;

    let mut cfg = Config::default();
    let file_name = path.clone().into_os_string().to_str().unwrap().to_string();
    cfg.file = Some(file_name.clone());
    let (_, header) = expected_header(&path);
    cfg.header_file = header;

    let ast = compiler::run_front(&cfg).unwrap();
    let asm = compiler::run_back(&cfg, ast);
    println!("  {}", path.display());

    for _ in 0..NUM_RUNS {

      // JIT Compilation and Interpretation
      let asm_clone = asm.clone();
      let start = Instant::now();
      let res = ProgContext::run(asm_clone, start, None);
      in_jit_run += start.elapsed().as_micros();
      debug_assert!(res == ReturnType::Return(*ret_val));

      // AOT Compilation to Assembly
      let asm_clone = asm.clone();
      let start = Instant::now();
      emit_x86(&file_name,asm_clone, false).unwrap();
      let emit_dur = start.elapsed();

      // Linking to Executable
      let mut linker = Command::new("gcc");
      linker.arg("-o").arg(format!("{}.exe", file_name))
        .arg(format!("{}.s", file_name))
        .arg("../runtime/run411.c");

      let start = Instant::now();
      let res = linker.status().expect("Failed to Link");
      let link_dur = start.elapsed();
      in_aot_compile += (emit_dur + link_dur).as_micros();
      assert!(res.success());

      // AOT Running
      let mut exe = Command::new(format!("./{}.exe", file_name));
      let start = Instant::now();
      exe.stdout(Stdio::null()).spawn().expect("Failed to Execute");
      in_aot_run += start.elapsed().as_micros();
    }

    bench_res.push((in_jit_run / NUM_RUNS, in_aot_compile / NUM_RUNS, in_aot_run / NUM_RUNS));
  }

  println!("-- Results --");
  println!("JIT:     Average Time Taken to Run the Test Case w/ JIT Compilation");
  println!("AOT:         Average Total Time to Compile, Link and Run Test Case w/ AOT. Total AOT = AOT Compile + AOT Run");
  println!("AOT Compile: Average Time Taken to (Normal) Compile and Link the Test Case to Executable");
  println!("AOT Run:     Average Time Taken to Run the Test Case's Executable");
  println!("% Diff:      Percent Speedup Or Slowdown for JIT in Comparison to AOT. % Diff = (AOT - JIT) / JIT");
  println!("% Run Diff:  Percent Speedup Or Slowdown for JIT in Comp to AOT for Runtime. % Diff = (AOT Run - JIT) / JIT");
  println!("JIT vs AOT:  While Compilation Method is Faster for the Test Case");

  let mut table = Table::new();
  table.set_format(FormatBuilder::new()
    .column_separator('│')
    .borders('│')
    .separators(&[LinePosition::Top], LineSeparator::new('─', '┬', '┌', '┐'))
    .separators(&[LinePosition::Intern], LineSeparator::new('─', '┼', '├', '┤'))
    .separators(&[LinePosition::Bottom], LineSeparator::new('─', '┴', '└', '┘'))
    .padding(1, 1)
    .build());
  table.add_row(row!["Test Case", "JIT (µs)", "AOT (µs)", "AOT Compile (µs)", "AOT Run (µs)", "% Diff", "% Run Diff", "JIT vs AOT"]);

  for ((path, _), (jrun, acomp, arun)) in return_tests.into_iter().zip_eq(bench_res.into_iter()) {
    let asum = acomp + arun;
    let per_total = (asum as i128 - jrun as i128) as f64 / jrun as f64 * 100.0;
    let per_run = (arun as i128 - jrun as i128) as f64 / jrun as f64 * 100.0;

    table.add_row(row![path.display(), jrun, asum, acomp, arun, 
      format!("{:.2}", per_total), format!("{:.2}", per_run),
      if jrun < asum { "JIT" } else { "AOT" }
    ]);
  }

  table.printstd();
}
