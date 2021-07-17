use std::fmt::{Display, Error, Formatter};

use derives::DebugFromDisplay;
use itertools::Itertools;

use crate::ops::{BinOp, UnOp};


// TODO: Consider extending to have named temporaries
// Instead of t0, t1, could have t_retval or something similar
// If allowed, maybe should use a different prefix (LLVM uses %)
#[derive(DebugFromDisplay, Clone)]
pub struct Temp(pub u64);

impl Display for Temp {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "t{}", self.0)
  }
}


// TODO: Consider adding types to operands or instructions
// Would be useful for Lab 4 if we plan on supporting it
// Can use an argument to disable for previous labs, which would set all to i32
#[derive(DebugFromDisplay, Clone)]
pub enum Operand {
  Temp(Temp),
  Const(i32),
}

impl Display for Operand {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    match self {
      Self::Temp(temp) => write!(f, "{}", temp),
      Self::Const(val) => write!(f, "{}", val),
    }
  }
}


#[derive(Debug, Clone)]
pub enum Instr {
  BinOp {
    op: BinOp,
    dest: Temp,
    src1: Operand,
    src2: Operand,
  },

  UnOp {
    op: UnOp,
    dest: Temp,
    src: Operand,
  },

  Mov {
    dest: Temp,
    src: Operand,
  },
  
  // Not all function calls return a value
  Call {
    name: String,
    dest: Option<Temp>,
    src: Vec<Operand>,
  },
}

impl Display for Instr {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    match self {
      Self::BinOp { op, dest, src1, src2 } =>
        write!(f, "{} = {} {} {}", dest, src1, op, src2),
      Self::UnOp { dest, op, src } => write!(f, "{} = {}{}", dest, op, src),
      Self::Mov { dest, src } => write!(f, "{} = {}", dest, src),
      Self::Call { dest: Some(dest), src, name } =>
        write!(f, "{} = {}({:?})", dest, name, src),
      Self::Call { dest: None, src, name } =>
        write!(f, "{}({})", name, src.iter().format(", ")),
    }
  }
}
