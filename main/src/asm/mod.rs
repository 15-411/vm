pub mod instr;
pub mod blocks;


use std::collections::HashMap;

use blocks::Func;


pub type ASM = HashMap<String, Func>;
