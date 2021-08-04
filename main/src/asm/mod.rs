pub mod instr;
pub mod blocks;


use fxhash::FxHashMap;

use blocks::Func;


pub type ASM = FxHashMap<String, Func>;
