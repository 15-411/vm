use std::fmt::{Display, Error, Formatter};

use derives::DebugFromDisplay;
use fxhash::FxHashMap;
use itertools::Itertools;

use crate::ops::BinOp;
use super::instr::{Operand, Instr, Temp};


#[derive(Debug, Clone)]
pub enum Cond {
  BinOp(Operand, BinOp, Operand),
  Value(Operand),
}

impl Display for Cond {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    match self {
      Self::BinOp(lop, opcode, rop) => write!(f, "{} {} {}", lop, opcode, rop),
      Self::Value(op) => write!(f, "{}", op),
    }
  }  
}


#[derive(DebugFromDisplay, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct BlockID(pub u64);

impl Display for BlockID {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "@{}", self.0)
  }
}


#[derive(Debug, Clone)]
pub enum BranchKind {
  Cond(Cond, BlockID, BlockID),
  Jump(BlockID),
  Ret(Option<Operand>),
}

impl Display for BranchKind {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    match self {
      Self::Cond(cond, true_block, false_block) =>
        write!(f, "cmp {} {} {}", cond, true_block, false_block),
      Self::Jump(block) => write!(f, "jmp {}", block),
      Self::Ret(None) => write!(f, "ret"),
      Self::Ret(Some(val)) => write!(f, "ret {}", val),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Branch {
  pub kind: BranchKind,
  pub line: u64,
}

impl Display for Branch {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "{}", self.kind)
  }
}


#[derive(Debug, Clone)]
pub struct BasicBlock {
  pub id: BlockID,
  pub preds: Vec<BlockID>,
  pub lines: Vec<Instr>,
  pub branch: Branch,
  pub line_start: u64,
}

impl Display for BasicBlock {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    writeln!(f, "{:0>2}    {} ({}):", self.line_start, self.id, self.preds.iter().format(", "))?;

    for instr in self.lines.iter() {
      writeln!(f, "{:0>2}      {}", instr.line, instr)?;
    }

    writeln!(f, "{:0>2}      {}", self.branch.line, self.branch)
  }
}


pub struct Func {
  pub name: String,
  pub params: Vec<Temp>,
  pub blocks: FxHashMap<BlockID, BasicBlock>,
  pub line_start: u64,
}

impl Display for Func {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    writeln!(f, "{:0>2}  {} {}", self.line_start, self.name, self.params.iter().format(" "))?;

    for (_, block) in self.blocks.iter() {
      writeln!(f, "{}", block)?;
    }

    Ok(())
  }
}
