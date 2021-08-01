use std::fmt::{Display, Error, Formatter};
use derives::DebugFromDisplay;


/// Available Binary Operations
#[derive(Copy, Clone, DebugFromDisplay, Hash, PartialEq, Eq)]
pub enum BinOp {
  Add,       // +
  Sub,       // -
  Mul,       // *
  Div,       // /
  Mod,       // %
  LShift,    // <<
  RShift,    // >>
  RShiftLog, // >>>

  Eq,        // ==
  Neq,       // !=
  Less,      // <
  Leq,       // <=
  Greater,   // >
  Geq,       // >=

  LogOr,     // ||
  LogAnd,    // &&
  BitOr,     // |
  BitXor,    // ^
  BitAnd,    // &
}

impl BinOp {
  /// Compute the binary operation
  /// If the operation would result in undefined behavior, returns None
  pub fn eval(&self, a: i32, b: i32) -> Option<i32> {
    use BinOp::*;
    
    match self {
      Add => Some(a.overflowing_add(b).0),
      Sub => Some(a.overflowing_sub(b).0),
      Mul => Some(a.overflowing_mul(b).0),
      Div => (!((b == 0) || (a == i32::MIN && b == -1))).then(|| a.overflowing_div(b).0),
      Mod => (!((b == 0) || (a == i32::MIN && b == -1))).then(|| a.overflowing_rem(b).0),
      LogOr => Some(a | b),
      LogAnd => Some(a & b), 
      BitOr => Some(a | b),
      BitXor => Some(a ^ b),
      BitAnd => Some(a & b),
      Eq => Some((a == b) as i32), 
      Neq => Some((a != b) as i32),
      Less => Some((a < b) as i32), 
      Leq => Some((a <= b) as i32), 
      Greater => Some((a > b) as i32), 
      Geq => Some((a >= b) as i32),    
      LShift => a.checked_shl(b as u32),  
      RShift => a.checked_shr(b as u32),
      RShiftLog => (a as u32).checked_shr(b as u32).map(|x| x as i32),
    }
  }
}

impl Display for BinOp {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    use BinOp::*;
    
    match self {
      LogOr     => write!(f, "||"),
      LogAnd    => write!(f, "&&"),
      BitOr     => write!(f, "|"),
      BitXor    => write!(f, "^"),
      BitAnd    => write!(f, "&"),
      Eq        => write!(f, "=="),
      Neq       => write!(f, "!="),
      Less      => write!(f, "<"),
      Leq       => write!(f, "<="),
      Greater   => write!(f, ">"),
      Geq       => write!(f, ">="),
      LShift    => write!(f, "<<"),
      RShift    => write!(f, ">>"),
      RShiftLog => write!(f, ">>>"),
      Mul       => write!(f, "*"),
      Div       => write!(f, "/"),
      Mod       => write!(f, "%"),
      Add       => write!(f, "+"),
      Sub       => write!(f, "-"),
    }
  }
}


/// Available Unary Operations
#[derive(Copy, Clone, DebugFromDisplay, Hash, PartialEq, Eq)]
pub enum UnOp {
  Neg,    // -
  LogNot, // !
  BitNot, // ~
}

impl UnOp {
  pub fn eval(&self, a: i32) -> i32 {
    match self {
      UnOp::Neg => a.overflowing_neg().0,
      UnOp::BitNot => !a,
      UnOp::LogNot => a^1,
    }
  }
}

impl Display for UnOp {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    use UnOp::*;

    match *self {
      Neg    => write!(f, "-"),
      LogNot => write!(f, "!"),
      BitNot => write!(f, "~"),
    }
  }
}
