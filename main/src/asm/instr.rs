use std::fmt::{Display, Error, Formatter};

use derives::DebugFromDisplay;
use itertools::Itertools;

use crate::ops::{BinOp, UnOp};

use super::reg::Register;


#[derive(DebugFromDisplay, Clone, Hash, PartialEq, Eq)]
pub enum TempID {
  Num(u64),
  Reg(Register),
}

impl Display for TempID {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    match self {
      Self::Num(val) => write!(f, "#{}", val),
      Self::Reg(val) => write!(f, "#{}", val),
    }
  }
}

#[derive(DebugFromDisplay, Clone, Hash, PartialEq, Eq)]
pub struct Temp(pub TempID);

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
  
  Phi {
    dest: Temp,
    srcs: Vec<Operand>,
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
      Self::Phi { dest, srcs } => write!(f, "{} = phi{}", dest, srcs.iter().format(" ")),
      Self::Call { dest: Some(dest), src, name } =>
        write!(f, "{} = {}({:?})", dest, name, src),
      Self::Call { dest: None, src, name } =>
        write!(f, "{}({})", name, src.iter().format(", ")),
    }
  }
}
