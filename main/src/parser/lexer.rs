use logos::Logos;


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
  #[regex(r"t(0|[1-9][0-9]*)", |lex| lex.slice().parse())] 
  Temp(u64),

  #[regex(r"B(0|[1-9][0-9]*)", |lex| lex.slice().parse())] 
  Block(u64),

  // TODO: Support Negative Numbers as well
  #[regex(r"0|[1-9][0-9]*", |lex| lex.slice().parse())] 
  Const(u64),

  #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().parse())] 
  Id(String),

  #[token("\n")] NewLine,
  #[regex(r"[ \t\f]+", logos::skip)]
  #[error]
  Error,
}
