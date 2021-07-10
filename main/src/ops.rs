use std::fmt::{Display, Error, Formatter};
use derives::DebugFromDisplay;


/// Available Unary Operations
#[derive(Copy, Clone, DebugFromDisplay, Hash, PartialEq, Eq)]
pub enum UnOp {
  Neg,    // -
  LogNot, // !
  BitNot, // ~
}

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
