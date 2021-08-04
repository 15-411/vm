use crate::ops::{UnOp, BinOp};

use super::lexer::Token;
use super::error::{errs, Result};


/// Convert BinOp Tokens to Associated OpCode
pub fn binop_code(op_token: Token) -> Result<BinOp> {
  use BinOp::*;
  match op_token {
    Token::Add      => Ok(Add),
    Token::Sub      => Ok(Sub),
    Token::Mul      => Ok(Mul),
    Token::Div      => Ok(Div),
    Token::Mod      => Ok(Mod),
    Token::LShift   => Ok(LShift),
    Token::RShift   => Ok(RShift),
    Token::Eq       => Ok(Eq),
    Token::Neq      => Ok(Neq),
    Token::Less     => Ok(Less),
    Token::Leq      => Ok(Leq), 
    Token::Greater  => Ok(Greater),
    Token::Geq      => Ok(Geq),
    Token::BitAnd   => Ok(BitAnd),
    Token::BitXor   => Ok(BitXor),
    Token::BitOr    => Ok(BitOr),
    Token::LogAnd   => Ok(LogAnd),
    Token::LogOr    => Ok(LogOr),
    _               => errs(format!("Invalid token as operator {:?}", op_token)),
  }
}

/// Convert UnOp Tokens to Associated OpCode
pub fn unop_code(op_token: Token) -> Result<UnOp> {
  match op_token {
    Token::Sub    => Ok(UnOp::Neg),
    Token::LogNot => Ok(UnOp::LogNot),
    Token::BitNot => Ok(UnOp::BitNot),
    _             => errs(format!("Invalid Unary Operand {:?}", op_token)),
  }
}