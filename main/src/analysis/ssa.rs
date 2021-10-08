use fxhash::FxHashMap;

use crate::asm::blocks::Func;
use crate::asm::{ASM, blocks::BlockID};
use crate::asm::instr::Temp;

use super::{SemError, SemResult};


// pub fn get_def_use<T>(asm: &CFG<T>) -> FxMap<T, FxSet<(BlockID, Location)>> where T: Hash + Eq + Clone {
//   let mut out_map: FxMap<T, FxSet<(BlockID, Location)>> = FxMap::default();

//   for (cur_block_id, cur_block) in asm.blocks() {
//     for (line_no, instr) in cur_block.lines.iter().enumerate() {
//       let srcs = instr.srcs();
//       for src in srcs {
//         out_map.entry(src)
//           .and_modify(|s| { s.insert((cur_block_id, Location::Line(line_no))); })
//           .or_insert(set!((cur_block_id, Location::Line(line_no))));
//       }
//     }

//     for src in cur_block.path.uses() {
//       out_map.entry(src)
//         .and_modify(|s| { s.insert((cur_block_id, Location::Branch)); })
//         .or_insert(set!((cur_block_id, Location::Branch)));
//     }
//   }

//   out_map
// }

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum BlockLoc {
  Branch,
  Line(u64)
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Loc(pub BlockID, pub BlockLoc);

impl Loc {
  fn to_line(&self, func: &Func) -> u64 {
    let block = func.blocks.get(&self.0).unwrap();
    match &self.1 {
      BlockLoc::Branch => block.branch.line,
      BlockLoc::Line(line) => block.lines[*line as usize].line,
    }
  }
}

struct DefUseBuilder {
  def_map: FxHashMap<Temp, Loc>,
  // use_map: FxHashMap<Temp, FxHashSet<Loc>>,
}

impl DefUseBuilder {
  fn new() -> Self {
    Self {
      def_map: FxHashMap::default(),
      // use_map: FxHashMap::default(),
    }
  }

  fn new_def(&mut self, temp: &Temp, loc: Loc) -> SemResult {
    if let Some(old_loc) = self.def_map.get(temp) {
      return Err(SemError::MultiDefs(temp.clone(), old_loc.clone(), loc));
    }

    self.def_map.insert(temp.clone(), loc);
    Ok(())
  }

  // Note: These seem unnecessary since we can find out if there is a missing definiton while executing
  // fn add_use(&mut self, temp: &Temp, loc: Loc) {
  //   self.use_map.entry(temp.clone())
  //     .and_modify(|s| { s.insert(loc); })
  //     .or_insert_with(|| set!(loc) );
  // }

  // fn missing_def(&self) -> SemResult {
  //   for (temp, locs) in self.use_map.iter() {
  //     if !self.def_map.contains_key(temp) {
  //       return Err(SemError::NoDef(temp.clone(), locs.iter().next().unwrap().clone()));
  //     }
  //   }

  //   Ok(()) 
  // }
}


pub fn ssa_form(abs: &ASM) -> SemResult {
  for func in abs.values() {
    let mut def_map = DefUseBuilder::new();

    for (bid, block) in func.blocks.iter() {
      for (line_no, instr) in block.lines.iter().enumerate() {
        if let Some(dest) = instr.dest() {
          def_map.new_def(dest, Loc(bid.clone(), BlockLoc::Line(line_no.clone() as u64)))?;
        }
      }
    }
  }

  Ok(())
}
