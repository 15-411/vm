use std::ops::Range;

use crate::error::ErrorTrait;
use super::lexer::Token;


#[derive(Debug)]
pub enum ParseErrorKind {
  UnknownInstr(String),
  InvalidFuncName(String),
  FuncNeedBlock,

  NoMatch(Token, Token),
  NoTemp(Token),
  NoBlock(Token),
  NoName(Token),
  InvalidOperand,
  EOF
}

#[derive(Debug)]
pub struct ParseError(pub ParseErrorKind, pub Range<usize>);


impl ErrorTrait for ParseError {
  fn code(&self) -> u64 {
    match &self.0 {
      ParseErrorKind::UnknownInstr(_) => 0,
      ParseErrorKind::InvalidFuncName(_) => 1,
      ParseErrorKind::FuncNeedBlock => 2,
      ParseErrorKind::NoMatch(_, _) => 94,
      ParseErrorKind::NoTemp(_) => 95,
      ParseErrorKind::NoBlock(_) => 96,
      ParseErrorKind::NoName(_) => 97,
      ParseErrorKind::InvalidOperand => 98,
      ParseErrorKind::EOF => 99,
    }
  }

  fn message(&self) -> &'static str {
    match &self.0 {
      ParseErrorKind::UnknownInstr(_) => "Unknown or Invalid Instruction(s)",
      ParseErrorKind::InvalidFuncName(_) => "Invalid Expected Function Name(s)",
      ParseErrorKind::FuncNeedBlock => "Function(s) Need at Least 1 Block",

      ParseErrorKind::NoMatch(_, _) => "No Match",
      ParseErrorKind::NoTemp(_) => "Require a Temp Label",
      ParseErrorKind::NoBlock(_) => "Require a Block Label",
      ParseErrorKind::NoName(_) => "Require a Name Label",
      ParseErrorKind::InvalidOperand => "Invalid Operand",
      ParseErrorKind::EOF => "Reached End of File",
    }
  }

  fn label(&self) -> Option<(String, Range<usize>)> {
    Some((match &self.0 {
      ParseErrorKind::UnknownInstr(instr) => format!("`{}` is not a valid instruction", instr),
      _ => "TODO".to_string()
    }, self.1.clone()))

    // match &self.0 {
    //   // ParseErrorKind::UnknownInstr(instr) => format!("`{}` is not a valid instruction", instr),
    //   // ParseErrorKind::InvalidFuncName(_) => "Invalid Expected Function Name(s)".to_string(),
    //   // ParseErrorKind::FuncNeedBlock => "Function(s) Need at Least 1 Block".to_string(),

    //   _ => None
    //   // ParseErrorKind::NoMatch(_, _) => "No Match".to_string(),
    //   // ParseErrorKind::NoTemp(_) => "Require a Temp Label".to_string(),
    //   // ParseErrorKind::NoBlock(_) => "Require a Block Label".to_string(),
    //   // ParseErrorKind::NoName(_) => "Require a Name Label".to_string(),
    //   // ParseErrorKind::InvalidOperand => "Invalid Operand".to_string(),
    //   // ParseErrorKind::EOF => "Reached End of File".to_string(),
    // }      
  }

  fn note(&self) -> Option<String> {
    match &self.0 {
      ParseErrorKind::UnknownInstr(_) => Some("See the `FORMAT.md` for a list of valid instructions.".to_string()),
      ParseErrorKind::InvalidFuncName(_) => Some("Invalid Expected Function Name(s)".to_string()),
      ParseErrorKind::FuncNeedBlock => Some("Function(s) Need at Least 1 Block".to_string()),

      ParseErrorKind::NoMatch(_, _) => None,
      ParseErrorKind::NoTemp(_) => None,
      ParseErrorKind::NoBlock(_) => None,
      ParseErrorKind::NoName(_) => None,
      ParseErrorKind::InvalidOperand => None,
      ParseErrorKind::EOF => Some("Reached End of File".to_string()),
    }
  }
}

pub type ParseResult<T> = Result<T, ParseError>;
