use fxhash::FxHashMap;

use crate::asm::ASM;
use crate::asm::blocks::{BasicBlock, BlockID, Branch, BranchKind, Func, Cond};
use crate::asm::instr::{Instr, InstrKind, Operand, Temp, TempID};

struct Context {
  pub counter: u64,
  mapping: FxHashMap<u64, u64>,
}

impl Context {
  fn new() -> Self {
    Self {
      counter: 0,
      mapping: FxHashMap::default(),
    }
  }

  fn map_temp(&mut self, temp: &Temp) {
    if let TempID::Num(idx) = temp.0 {
      self.mapping.insert(idx, self.counter);
      self.counter += 1;
    }
  }

  fn get_map(&self, temp: Temp) -> Temp {
    match temp {
      Temp(TempID::Num(idx)) => Temp(TempID::Num(*self.mapping.get(&idx).unwrap())),
      val => val,
    }
  }

  fn get_map_op(&self, op: Operand) -> Operand {
    match op {
      Operand::Temp(temp) => Operand::Temp(self.get_map(temp)),
      other => other,
    }
  }

  fn map_dests(&mut self, params: &Vec<Temp>, blocks: &FxHashMap<BlockID, BasicBlock>) {
    for param in params {
      self.map_temp(param);
    }

    for BasicBlock { lines, .. } in blocks.values() {
      for line in lines {
        if let Some(temp) = line.dest() {
          self.map_temp(temp);
        }
      }
    }
  }

  fn rename_temps(&self, params: Vec<Temp>, blocks: FxHashMap<BlockID, BasicBlock>) -> (Vec<Temp>, FxHashMap<BlockID, BasicBlock>) {
    (
      params.into_iter().map(|param| self.get_map(param)).collect(),
      blocks.into_iter().map(|(bidx, BasicBlock { id, preds, lines, branch, line_start })| {
        (bidx, BasicBlock { id, preds, line_start,
          lines: lines.into_iter().map(|Instr { line, kind }| Instr { line, kind: match kind {
            InstrKind::BinOp { dest, op, src1, src2 } => InstrKind::BinOp {
              dest: self.get_map(dest),
              op,
              src1: self.get_map_op(src1),
              src2: self.get_map_op(src2),
            },

            InstrKind::UnOp { dest, op, src } => InstrKind::UnOp {
              dest: self.get_map(dest),
              op,
              src: self.get_map_op(src),
            },

            InstrKind::Mov { dest, src } => InstrKind::Mov {
              dest: self.get_map(dest),
              src: self.get_map_op(src),
            },

            InstrKind::Phi { dest, srcs } => InstrKind::Phi {
              dest: self.get_map(dest),
              srcs: srcs.into_iter().map(|src| self.get_map_op(src)).collect()
            },

            InstrKind::Call { name, dest, src } => InstrKind::Call {
              name,
              dest: dest.map(|dest| self.get_map(dest)),
              src: src.into_iter().map(|src| self.get_map_op(src)).collect(),
            },

            InstrKind::If { cond, block } => InstrKind::If {
              block,
              cond: self.get_map_op(cond),
            },

            InstrKind::Print { value } => InstrKind::Print { value: self.get_map_op(value) },
            InstrKind::Dump => InstrKind::Dump,
            InstrKind::Nop => InstrKind::Nop,

          }}).collect(),

          branch: {
            let Branch { line, kind } = branch;
            Branch { line, kind: match kind {
              BranchKind::Ret(src) => BranchKind::Ret(src.map(|src| self.get_map_op(src))),
              BranchKind::Cond(cond, bidx1, bidx2) => BranchKind::Cond(
                match cond {
                  Cond::Value(src) => Cond::Value(self.get_map_op(src)),
                  Cond::BinOp(src1, op, src2) =>
                    Cond::BinOp(self.get_map_op(src1), op, self.get_map_op(src2)),
                },

                bidx1,
                bidx2
              ),
              other => other
            }}
          },
        })
      }).collect()
    )
  }
}

pub fn rename(abs: ASM) -> ASM {
  abs.into_iter().map(|(fname, Func { name, params, blocks, line_start, .. })| {
    
    let mut ctx = Context::new();
    ctx.map_dests(&params, &blocks);
    let (params, blocks) = ctx.rename_temps(params, blocks);
    (fname, Func { name, line_start, params, blocks, count: Some(ctx.counter)})

  }).collect()
}
