use std::time::Instant;

use chrono::Local;
use fxhash::FxHashMap;
use itertools::Itertools;

use crate::asm::ASM;
use crate::asm::blocks::{Func, BasicBlock, BranchKind, Cond, CondJumpKind};
use crate::asm::instr::{Temp, Operand, InstrKind, TempID};
use crate::asm::reg::Register;


struct TempStore {
  pub temps: FxHashMap<Temp, i32>,
  pub base: Vec<i32>,
}

impl TempStore {
  fn new(count: u64) -> Self {
    let mut temps = FxHashMap::default();
    for reg in Register::ALL {
      temps.insert(Temp(TempID::Reg(reg)), 0);
    }

    TempStore { temps, base: vec![0; count as usize] }    
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
      Operand::Temp(temp @ Temp(TempID::Reg(_))) => *self.temps.get(temp).unwrap(),
      Operand::Temp(Temp(TempID::Num(elem))) => self.base[*elem as usize],
      //Operand::Temp(temp) => *self.temps.get(temp).unwrap(),
    }
  }

  fn save(&mut self, dest: &Temp, src: i32) {
    match &dest.0 {
      TempID::Reg(_) => {
        *self.temps.get_mut(dest).unwrap() = src;
      },

      TempID::Num(elem) => {
        self.base[*elem as usize] = src;
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
  Timeout,
}

pub struct ProgContext {
  prog: ASM,
  start: Instant,
  timeout: u64,
  zero_flag: bool,
  sign_flag: bool,
  overflow_flag: bool,
}

impl ProgContext {
  fn run_func(&self, name: String, args: Vec<i32>) -> ReturnType {
    let Func { params, blocks, count, .. } = self.prog.get(&name).unwrap();
    let mut prev_block = None;
    let mut curr_block = blocks.keys().min().unwrap().clone();
    let mut store = TempStore::new(count.unwrap());

    // Insert Arguments as Params
    for (param, arg) in params.iter().zip_eq(args.into_iter()) {
      store.temps.insert(param.clone(), arg);
    }

    // Run Function Blocks
    'outer: loop {
      if self.start.elapsed().as_secs() > self.timeout {
        return ReturnType::Timeout;
      }

      let BasicBlock { preds, lines, branch, .. } 
        = blocks.get(&curr_block).unwrap();

      // Evaluate Operations
      for line in lines {
        match &line.kind {
          InstrKind::BinOp { op, dest, src1, src2 } => {
            let src1_val = store.get(src1);
            let src2_val = store.get(src2);
            store.save(dest, match op.eval(src1_val, src2_val) {
              Some(val) => val,
              None => return ReturnType::DivByZero,
            });
          },

          InstrKind::UnOp  { op, dest, src } => {
            let dest_val = op.eval(store.get(src));
            store.save(dest, dest_val);
          },

          InstrKind::Mov   { dest, src } => {
            let src_val = store.get(src);
            store.save(dest, src_val);
          },

          InstrKind::If    { cond, block } => {
            if store.get(cond) != 0 { 
              prev_block = Some(curr_block);
              curr_block = *block;
              continue 'outer;
            }
          },

          InstrKind::Phi   { dest, srcs } => {
            if let Some(prev) = prev_block {
              let pred_idx = preds.iter().position(|&x| x == prev).unwrap();
              let src = srcs.get(pred_idx).unwrap();
              store.save(dest, store.get(src));
    
            } else {
              panic!("First Block Executed has Phi Functions");
            }
          },

          InstrKind::Call  { name, dest, src } => {
            match self.run_func(name.clone(), src.iter().map(|x| store.get(x)).collect()) {
              ReturnType::Return(val) => if let Some(dest) = dest {
                store.save(dest,  val);
              },
              other => return other,
            }
          },

          InstrKind::Print { value } => {
            println!("[{}] Line {}: {} = {}", Local::now().time().format("%H:%M:%S"), line.line, value, store.get(value));
          },

          InstrKind::Dump => {
            println!("[{}] Line {}: Dump of All Temps", line.line, Local::now().time().format("%H:%M:%S"));
            store.dump();
          },

          InstrKind::Nop => {},
        }
      }

      // Path Handling
      match &branch.kind {
        BranchKind::Ret(None) => return ReturnType::Return(0),  // Doesnt Matter if No Dest
        BranchKind::Ret(Some(ret)) => return ReturnType::Return(store.get(ret)),
        BranchKind::Jump(bidx) => { 
          prev_block = Some(curr_block);
          curr_block = *bidx;
        },

        BranchKind::Cond(cond, tidx, fidx) => {
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

        BranchKind::CondJump(cond, tidx, fidx) => {
          let val = match cond {
            CondJumpKind::Zero | CondJumpKind::Equal => self.zero_flag,
            CondJumpKind::NotZero | CondJumpKind::NotEqual => !self.zero_flag,
            CondJumpKind::Less | CondJumpKind::NotGreaterEqual => self.sign_flag != self.overflow_flag,
            CondJumpKind::LessEqual | CondJumpKind::NotGreater => (self.sign_flag != self.overflow_flag) || self.zero_flag,
            CondJumpKind::Greater | CondJumpKind::NotLessEqual => (self.sign_flag == self.overflow_flag) && !self.zero_flag,
            CondJumpKind::GreaterEqual | CondJumpKind::NotLess => self.sign_flag == self.overflow_flag,
          };

          let block = if val { tidx } else { fidx };
          prev_block = Some(curr_block);
          curr_block = *block;
        },
      }
    }
  }

  pub fn run(prog: ASM, timeout: u64) -> ReturnType {
    let ctx = ProgContext {
      prog, start: Instant::now(), timeout,
      zero_flag: false, overflow_flag: false, sign_flag: false,
    };

    ctx.run_func("main".to_string(), vec![])
  }  
}
