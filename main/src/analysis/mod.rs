mod ssa;

use std::ops::Range;

use crate::{asm::{ASM, instr::Temp}, error::ErrorTrait};

use ssa::{ssa_form, Loc};


pub enum SemError {
  NoMain,
  InvalidCFG,
  MultiDefs(Temp, Loc, Loc),
  NoDef(Temp, Loc),
}

impl ErrorTrait for SemError {
  fn code(&self) -> u64 {
    match self {
      Self::NoMain => 0,
      Self::InvalidCFG => 1,
      Self::MultiDefs(_, _, _) => 2,
      Self::NoDef(_, _) => 3,
    }
  }

  fn message(&self) -> &'static str {
    match self {
      Self::NoMain => "Missing Function `main`",
      Self::InvalidCFG => "Invalid CFG TODO",
      _ => "TODO",
      // Self::MultiDefs(temp, _, _) => format!("Temporary `{}` is Defined Multiple Times", temp),
      // Self::NoDef(temp, _) => format!("Temporary `{}` is Never Defined", temp),
    }
  }

  fn label(&self) -> Option<(String, Range<usize>)> {
    None
  }

  fn note(&self) -> Option<String> {
    match self {
      Self::NoMain => Some("C0 VM needs a function called `main` to start executing at".to_string()),
      _ => None,
    }
  }
}

pub type SemResult = Result<(), SemError>;


fn has_main(abs: &ASM) -> SemResult {
  if abs.contains_key("main") { Ok(()) } else { Err(SemError::NoMain) }
}


// fn validate_cfg -> Check if the predecessors correspond with the successor construction
// fn check_phis -> Check if number of args to phi functions is equal to num predecessors

pub fn sem_analysis(abs: &ASM, ssa: bool) -> SemResult {
  has_main(abs)?;

  // SSA Checks
  if ssa {
    ssa_form(abs)?;
  }

  Ok(())
}
