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
  #[token("phi")]  Phi,

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

  #[token("\n")]   NewLine,

  // Identifiers
  #[regex(r"\#(0|[1-9][0-9]*)", parse_udec)] 
  Temp(u64),

  #[regex(r"@(0|[1-9][0-9]*)", parse_udec)] 
  Block(u64),

  #[regex(r"(-?)(0|[1-9][0-9]*)", parse_dec)]
  #[regex(r"0[xX][0-9a-fA-F]+", parse_hex)]
  Const(i32),

  #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().parse())] 
  Id(String),

  #[regex(r"[ \t\f\v\r]+", logos::skip)]
  #[regex(r"//[^\n]*", logos::skip)]
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

fn parse_hex(lex: &mut logos::Lexer<Token>) -> Option<i32> {
  // println!("LEX {:?}", lex.slice());

  fn preceding_zeros(string: &str) -> usize {
    for (i, chr) in string.char_indices() {
      if chr != '0' {
        return i;
      }
    }

    string.len()
  }

  let buffer = &lex.slice()[2..];
  if buffer.len() - preceding_zeros(buffer) > 8 {
    None
  } else {
    match i64::from_str_radix(&lex.slice()[2..], 16) {
      Ok(val) => Some(val as i32),
      Err(_) => None,
    }
  }
}


fn parse_udec(lex: &mut Lexer<Token>) -> Option<u64> {
  let slice = lex.slice();
  let n: u64 = slice[1..].parse().ok()?; // skip '#' or '@'
  Some(n)
}
