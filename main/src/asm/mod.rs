pub mod instr;
pub mod blocks;
pub mod reg;


use fxhash::FxHashMap;

use blocks::Func;


pub type ASM = FxHashMap<String, Func>;
