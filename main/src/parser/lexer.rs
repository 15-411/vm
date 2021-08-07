use std::str::FromStr;

use logos::{Logos, Lexer};


// TODO: Consider dynamic extensibility of lexer
// If we want to allow custom instructions, then we need to find some way
// to add onto the list of tokens. This might require using a custom solution
// instead of the Logos library.

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token {
  #[token("ret")]  Ret,
  #[token("jmp")]  Jmp,
  #[token("if")]   If,
  #[token("call")] Call,

  #[token("(")]    LParen,
  #[token(")")]    RParen,
  #[token(":")]    Colon,
  #[token("=")]    Assign,
  #[token(",")]    Comma,

  // Arithmetic Ops
  #[token("+")]    Add,
  #[token("-")]    Sub,
  #[token("*")]    Mul,
  #[token("/")]    Div,
  #[token("%")]    Mod,
  #[token("<<")]   LShift,
  #[token(">>")]   RShift,
  
  // Comparsion Ops
  #[token("==")]   Eq,
  #[token("!=")]   Neq,
  #[token("<")]    Less,
  #[token("<=")]   Leq,
  #[token(">")]    Greater,
  #[token(">=")]   Geq,

  // Boolean Ops
  #[token("&")]    BitAnd,
  #[token("^")]    BitXor,
  #[token("|")]    BitOr,
  #[token("~")]    BitNot,
  #[token("&&")]   LogAnd,
  #[token("||")]   LogOr,
  #[token("!")]    LogNot,

  // Identifiers
  #[regex(r"t(0|[1-9][0-9]*)", temp)] 
  Temp(u64),

  #[regex(r"B(0|[1-9][0-9]*)", block)] 
  Block(u64),

  #[regex(r"(-?)(0|[1-9][0-9]*)", parse_dec)] 
  Const(i32),

  #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().parse())] 
  Id(String),

  #[token("\n")] NewLine,
  #[regex(r"[ \t\f]+", logos::skip)]
  #[error]
  Error,
}

/// Parse Numeral Number Strings to i32 Integers
/// Must specially handle INT_MIN because the minus sign is ignored
fn parse_dec(lex: &mut Lexer<Token>) -> i32 {
  const INT_MIN: i64 = -2_147_483_648;
  const INT_MAX: i64 = 2_147_483_648;

  let res = match i64::from_str(lex.slice()) {
    Ok(val) if val >= INT_MIN && val <= INT_MAX => Some(val as i32),
    _ => None,
  };

  res.unwrap()
}

fn temp(lex: &mut Lexer<Token>) -> Option<u64> {
  let slice = lex.slice();
  let n: u64 = slice[1..].parse().ok()?; // skip 't'
  Some(n)
}

fn block(lex: &mut Lexer<Token>) -> Option<u64> {
  let slice = lex.slice();
  let n: u64 = slice[1..].parse().ok()?; // skip 'b'
  Some(n)
}
