use chrono::Local;
use fxhash::FxHashMap;
use itertools::Itertools;

use crate::asm::ASM;
use crate::asm::blocks::{Func, BlockID, BasicBlock, Branch, Cond};
use crate::asm::instr::{Temp, Operand, Instr, TempID};
use crate::asm::reg::Register;


struct TempStore {
  pub temps: FxHashMap<Temp, i32>,
}

impl TempStore {
  fn new() -> Self {
    let mut temps = FxHashMap::default();
    for reg in Register::ALL {
      temps.insert(Temp(TempID::Reg(reg)), 0);
    }

    TempStore { temps }    
  }

  // fn get_op64(&self, op: &Operand) -> u64 {
  //   match op {
  //     Operand::Const(val) => *val as u64,
  //     Operand::Temp(temp) => *self.temps.get(temp).unwrap(),
  //   }
  // }

  fn get(&self, op: &Operand) -> i32 {
    match op {
      Operand::Const(val) => *val,
      Operand::Temp(temp) => *self.temps.get(temp).unwrap(),
    }
  }

  fn update(&mut self, dest: &Temp, src: i32) {
    *self.temps.get_mut(dest).unwrap() = src;
  }

  fn save(&mut self, dest: &Temp, src: i32) {
    match &dest.0 {
      TempID::Reg(_) => {
        *self.temps.get_mut(dest).unwrap() = src;
      },

      TempID::Num(_) => {
        self.temps.insert(dest.clone(), src);
      },
    }
  }

  fn dump(&self) {
    for (name, value) in self.temps.iter().sorted_unstable() {
      println!("  {}\t= {}", name, value);
    }
  }
}


#[derive(Eq, PartialEq, Debug)]
pub enum ReturnType {
  Return(i32),
  DivByZero,
  // Abort,
  // MemError,
  // Timeout,
}

pub struct ProgContext {
  prog: ASM,
}

impl ProgContext {
  fn run_func(&self, name: String, args: Vec<i32>) -> ReturnType {
    let Func { params, blocks, .. } = self.prog.get(&name).unwrap();
    let mut prev_block = None;
    let mut curr_block = BlockID(0);
    let mut store = TempStore::new();

    // Insert Arguments as Params
    for (param, arg) in params.iter().zip_eq(args.into_iter()) {
      store.temps.insert(param.clone(), arg);
    }

    // Run Function Blocks
    'outer: loop {
      let BasicBlock { preds, lines, branch, .. } 
        = blocks.get(curr_block.0 as usize).unwrap();

      // Evaluate Operations
      for line in lines {
        match line {
          Instr::BinOp { op, dest, src1, src2 } => {
            let src1_val = store.get(src1);
            let src2_val = store.get(src2);
            store.save(dest, match op.eval(src1_val, src2_val) {
              Some(val) => val,
              None => return ReturnType::DivByZero,
            });
          },

          Instr::UnOp  { op, dest, src } => {
            let dest_val = op.eval(store.get(src));
            store.save(dest, dest_val);
          },

          Instr::Mov   { dest, src } => {
            let src_val = store.get(src);
            store.save(dest, src_val);
          },

          Instr::If    { cond, block } => {
            if store.get(cond) != 0 { 
              prev_block = Some(curr_block);
              curr_block = *block;
              continue 'outer;
            }
          },

          Instr::Phi   { dest, srcs } => {
            if let Some(prev) = prev_block {
              let pred_idx = preds.iter().position(|&x| x == prev).unwrap();
              let src = srcs.get(pred_idx).unwrap();
              store.update(dest, store.get(src));
    
            } else {
              panic!("First Block Executed has Phi Functions");
            }
          },

          Instr::Call  { name, dest, src } => {
            match self.run_func(name.clone(), src.iter().map(|x| store.get(x)).collect()) {
              ReturnType::Return(val) => if let Some(dest) = dest {
                store.save(dest,  val);
              },
              other => return other,
            }
          },

          Instr::Print { value } => {
            // TODO: Include Line Number
            println!("[{}] {} = {}", Local::now().time().format("%H:%M:%S"), value, store.get(value));
          },

          Instr::Dump => {
            // TODO: Include Line Number
            println!("[{}] Dump of All Temps", Local::now().time().format("%H:%M:%S"));
            store.dump();
          },
        }
      }

      // Path Handling
      match branch {
        Branch::Ret(None) => return ReturnType::Return(0),  // Doesnt Matter if No Dest
        Branch::Ret(Some(ret)) => return ReturnType::Return(store.get(ret)),
        Branch::Jump(bidx) => { 
          prev_block = Some(curr_block);
          curr_block = *bidx;
        },

        Branch::Cond(cond, tidx, fidx) => {
          let cond_val = match cond {
            Cond::BinOp(src1, op, src2) =>
              match op.eval(store.get(src1), store.get(src2)) {
                Some(val) => val,
                None => return ReturnType::DivByZero,
              },
            Cond::Value(src) => store.get(src),
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
