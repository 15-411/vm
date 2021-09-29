use std::fmt::{Display, Error, Formatter};

use derives::DebugFromDisplay;
use itertools::Itertools;

use crate::ops::{BinOp, UnOp};

use super::{blocks::BlockID, reg::Register};


#[derive(DebugFromDisplay, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TempID {
  Num(u64),
  Reg(Register),
}

impl Display for TempID {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    match self {
      Self::Num(val) => write!(f, "{}", val),
      Self::Reg(val) => write!(f, "{}", val),
    }
  }
}

#[derive(DebugFromDisplay, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Temp(pub TempID);

impl Display for Temp {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "#{}", self.0)
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
pub enum InstrKind {
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
  
  If {
    cond: Operand,
    block: BlockID,
  },

  Phi {
    dest: Temp,
    srcs: Vec<Operand>,
  },

  Call {
    name: String,
    dest: Option<Temp>,
    src: Vec<Operand>,
  },

  Print {
    value: Operand
  },

  Dump
}

impl Display for InstrKind {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    match self {
      Self::BinOp { op, dest, src1, src2, .. } =>
        write!(f, "{} = {} {} {}", dest, src1, op, src2),
      Self::UnOp { dest, op, src, .. } => write!(f, "{} = {}{}", dest, op, src),
      Self::Mov { dest, src, .. } => write!(f, "{} = {}", dest, src),
      Self::If { cond, block, .. } => write!(f, "if {} {}", cond, block),
      Self::Phi { dest, srcs, .. } => write!(f, "{} = phi {}", dest, srcs.iter().format(" ")),
      Self::Call { dest: Some(dest), src, name, .. } =>
        write!(f, "{} = call {} {}", dest, name, src.iter().format(" ")),
      Self::Call { dest: None, src, name, .. } =>
        write!(f, "call {} {}", name, src.iter().format(", ")),
      Self::Print { value, .. } => write!(f, "print {}", value),
      Self::Dump { .. } => write!(f, "dump"),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Instr {
  pub line: u64,
  pub kind: InstrKind,
}

impl Display for Instr {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "{}", self.kind)
  }
}
