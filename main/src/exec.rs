use fxhash::FxHashMap;
use itertools::Itertools;

use crate::asm::ASM;
use crate::asm::blocks::{Func, BlockID, BasicBlock, Branch, Cond};
use crate::asm::instr::{Temp, Operand, Instr};


struct TempStore {
  pub temps: FxHashMap<Temp, u64>,
}

impl TempStore {
  fn get(&self, temp: &Temp) -> u64 {
    *self.temps.get(temp).unwrap()
  }

  fn get_op64(&self, op: &Operand) -> u64 {
    match op {
      Operand::Const(val) => *val as u64,
      Operand::Temp(temp) => *self.temps.get(temp).unwrap(),
    }
  }

  fn get_op32(&self, op: &Operand) -> i32 {
    match op {
      Operand::Const(val) => *val,
      Operand::Temp(temp) => *self.temps.get(temp).unwrap() as i32,
    }
  }

  fn update(&mut self, dest: &Temp, src: u64) {
    *self.temps.get_mut(dest).unwrap() = src;
  }

  fn save(&mut self, dest: &Temp, src: u64) {
    // TODO: Can switch between SSA and non-SSA here
    self.temps.insert(dest.clone(), src);
  }
}


#[derive(Eq, PartialEq, Debug)]
pub enum ReturnType {
  Return(u64),
  DivByZero,
  Abort,
  MemError,
  Timeout,
}

pub struct ProgContext {
  prog: ASM,
}

impl ProgContext {
  fn run_func(&self, name: String, args: Vec<u64>) -> ReturnType {
    let Func { params, blocks, .. } = self.prog.get(&name).unwrap();
    let mut prev_block = None;
    let mut curr_block = BlockID(0);
    let mut store = TempStore { temps: FxHashMap::default() };

    // Insert Arguments as Params
    for (param, arg) in params.iter().zip_eq(args.into_iter()) {
      store.temps.insert(param.clone(), arg);
    }

    // Run Function Blocks
    loop {
      let BasicBlock { preds, phis, lines, branch, .. } 
        = blocks.get(curr_block.0 as usize).unwrap();

      // Evaluate Phi Functions
      if !phis.is_empty() {
        if let Some(prev) = prev_block {
          let pred_idx = preds.iter().position(|&x| x == prev).unwrap();
          for (dest, ops) in phis {
            let op = ops.get(pred_idx).unwrap();
            store.save(dest, store.get_op64(op));
          }

        } else {
          panic!("First Block Executed has Phi Functions"); // TODO: Better Method for Canceling
        }
      }

      // Evaluate Operations
      for line in lines {
        match line {
          Instr::BinOp { op, dest, src1, src2 } => {
            let src1_val = store.get_op32(src1);
            let src2_val = store.get_op32(src2);
            store.save(dest, match op.eval(src1_val, src2_val) {
              Some(val) => val as u64,
              None => return ReturnType::DivByZero,
            });
          },

          Instr::UnOp  { op, dest, src } => {
            let dest_val = op.eval(store.get_op32(src));
            store.save(dest, dest_val as u64);
          },

          Instr::Mov   { dest, src } => {
            let src_val = store.get_op64(src);
            store.save(dest, src_val);
          },

          Instr::Call  { name, dest, src } => {
            match self.run_func(name.clone(), src.iter().map(|x| store.get_op64(x)).collect()) {
              ReturnType::Return(val) => if let Some(dest) = dest {
                store.save(dest, val);
              },
              other => return other,
            }
          },
        }
      }

      // Path Handling
      match branch {
        Branch::Ret(None) => return ReturnType::Return(0),  // Doesnt Matter if No Dest
        Branch::Ret(Some(ret)) => return ReturnType::Return(store.get_op64(ret)),
        Branch::Jump(bidx) => { 
          prev_block = Some(curr_block);
          curr_block = *bidx;
        },

        Branch::Cond(cond, tidx, fidx) => {
          let cond_val = match cond {
            Cond::BinOp(src1, op, src2) =>
              match op.eval(store.get_op32(src1), store.get_op32(src2)) {
                Some(val) => val as u64,
                None => return ReturnType::DivByZero,
              },
            Cond::Value(src) => store.get_op64(src),
          };

          let block = if cond_val == 0 { fidx } else { tidx };
          prev_block = Some(curr_block);
          curr_block = *block;
        },
      }
    }
  }

  pub fn run(prog: ASM) -> ReturnType {
    let ctx = ProgContext { prog };
    ctx.run_func("main".to_string(), vec![])
  }  
}
