mod lexer;
pub mod error;
mod utils;

use fxhash::FxHashMap;
use logos::{Logos, Lexer};

use crate::asm::ASM;
use crate::asm::blocks::{Func, BasicBlock, BlockID, Branch, BranchKind, Cond, CondJumpKind};
use crate::asm::instr::{InstrKind, Instr, Operand, Temp};

use lexer::Token;
use error::{ParseError, ParseResult};
use utils::{unop_code, binop_code};

use self::error::ParseErrorKind;


#[derive(Clone)]
struct Parser<'a> {
  peeked: Option<Option<Token>>,
  // peeked_span: Option<Range<u64>>,
  // peeked_slice: Option<&'a str>,
  lexer: Lexer<'a, Token>,
  cur_line: u64,
}

impl<'a> Parser<'a> {
  fn err(&self, kind: ParseErrorKind) -> ParseError {
    ParseError(kind, self.lexer.span())
  }

  fn err_to_line_end(&mut self, kind: ParseErrorKind) -> ParseError {
    let mut range = self.lexer.span();

    while !matches!(self.peek(), Ok(Token::NewLine) | Err(ParseError(ParseErrorKind::EOF, _))) {
      self.skip().expect("Can Never Be EOF");
      range.end = self.lexer.span().end;
    }

    ParseError(kind, range)
  }


  // ---------------------------- HELPER FUNCTIONS ----------------------------
  /// Get the next token from the lexer
  /// Returns EOF when no more tokens
  fn token(&mut self) -> ParseResult<Token> {
    let tok = match self.peeked.take() {
      Some(v) => v,
      None => self.lexer.next(),
    };

    tok.ok_or(self.err(ParseErrorKind::EOF))
  }
  
  /// Look at the next token in the lexer stream.
  /// This function NEVER moves the lexer, and can be called multiple times
  /// and will return the same result.
  fn peek(&mut self) -> ParseResult<&Token> {
    let iter = &mut self.lexer;
    if let Some(peek) = self.peeked.get_or_insert_with(|| iter.next()).as_ref() {
      Ok(peek)
    } else {
      Err(ParseError(ParseErrorKind::EOF, self.lexer.span()))
    }
  }

  // Skips to the next token without checking what it is
  fn skip(&mut self) -> ParseResult<()> {
    self.token()?;
    Ok(())
  }

  fn skip_newlines(&mut self) -> ParseResult<()> {
    self.munch(Token::NewLine)?;
    self.cur_line += 1;

    self.skip_opt_newlines();
    Ok(())
  }

  fn skip_opt_newlines(&mut self) {
    while matches!(self.peek(), Ok(Token::NewLine)) {
      self.skip().expect("Should Always Work");
      self.cur_line += 1;
    }
  }

  // Skips to the next token, verifying that this has the value we expect.
  fn munch(&mut self, tok: Token) -> ParseResult<()> {
    let ltok = self.token()?;
    if ltok == tok {
      Ok(())
    } else {
      Err(self.err(ParseErrorKind::NoMatch(tok, ltok)))
    }
  }

  // Expect next token to be a temp and get inner count
  fn temp(&mut self) -> ParseResult<Temp> {
    let tok  = self.token()?;
    if let Token::Temp(val) = tok {
      Ok(Temp(val))
    } else {
      Err(self.err(ParseErrorKind::NoTemp(tok)))
    }
  }

  fn block(&mut self) -> ParseResult<BlockID> {
    let tok  = self.token()?;
    if let Token::Block(val) = tok {
      Ok(BlockID(val))
    } else {
      Err(self.err(ParseErrorKind::NoBlock(tok)))
    }
  }

  // Expect next token to be an ID and get inner name
  fn name(&mut self) -> ParseResult<String> {
    let tok = self.token()?;
    if let Token::Id(name) = tok {
      Ok(name)
    } else {
      Err(self.err(ParseErrorKind::NoName(tok)))
    }
  }


  // ---------------------------- PARSER FUNCTIONS ----------------------------
  fn operand(&mut self) -> ParseResult<Operand> {
    match self.token()? {
      Token::Temp(val) => Ok(Operand::Temp(Temp(val))),
      Token::Const(val) => Ok(Operand::Const(val as i32)),
      _ => Err(self.err(ParseErrorKind::InvalidOperand)),
    }
  }

  fn mov_binop_instr(&mut self, dest: Temp, lsrc: Operand) -> ParseResult<Instr> {
    match self.token()? {
      Token::NewLine => {
        self.cur_line += 1;
        Ok(Instr { kind: InstrKind::Mov { dest, src: lsrc }, line: self.cur_line - 1 })
      },
      
      tok => {
        let op = binop_code(tok, self.lexer.span())?;
        let src2 = self.operand()?;

        self.munch(Token::NewLine)?;
        self.cur_line += 1;
        Ok(Instr { kind: InstrKind::BinOp { dest, op, src1: lsrc, src2 }, line: self.cur_line - 1 })
      }
    }
  }

  fn instr(&mut self) -> ParseResult<Instr> {
    match self.token()? {
      Token::Temp(val) => {
        let dest = Temp(val);
        self.munch(Token::Assign)?;

        match self.token()? {
          op @ (Token::Sub | Token::LogNot | Token::BitNot) => {
            let src = self.operand()?;
            self.munch(Token::NewLine)?;
            self.cur_line += 1;

            Ok(Instr { 
              kind: InstrKind::UnOp { dest, src, op: unop_code(op, self.lexer.span())? }, 
              line: self.cur_line - 1 
            })
          },

          Token::Phi => {
            let mut srcs = vec![];
            while !matches!(self.peek()?, Token::NewLine) {
              match self.token()? {
                Token::Temp(val) => { srcs.push(Operand::Temp(Temp(val))); },
                Token::Const(val) => { srcs.push(Operand::Const(val)); },
                _ => unreachable!(),
              }
            }

            Ok(Instr { kind: InstrKind::Phi { dest, srcs }, line: self.cur_line })
          },

          Token::Call => {
            let name = self.name()?;
            let mut params = vec![];
            while !matches!(self.peek()?, Token::NewLine) {
              params.push(self.operand()?);
            }
            
            Ok(Instr { kind: InstrKind::Call { dest: Some(dest), name, src: params }, line: self.cur_line })
          },

          Token::Temp(val) =>
            self.mov_binop_instr(dest, Operand::Temp(Temp(val))),
          Token::Const(val) =>
            self.mov_binop_instr(dest, Operand::Const(val)),
          
          _ => Err(self.err_to_line_end(ParseErrorKind::UnknownInstr(self.lexer.slice().to_string()))),
        }
      },

      Token::If => {
        let cond = self.operand()?;
        let block = self.block()?;
        Ok(Instr { kind: InstrKind::If { cond, block }, line: self.cur_line })
      },

      Token::Print => {
        let value = self.operand()?;
        Ok(Instr { kind: InstrKind::Print { value }, line: self.cur_line })
      },

      Token::Dump => {
        Ok(Instr { kind: InstrKind::Dump, line: self.cur_line })
      },

      Token::Nop => {
        Ok(Instr { kind: InstrKind::Nop, line: self.cur_line })
      },

      Token::Call => {
        let name = self.name()?;
        let mut params = vec![];
        while !matches!(self.peek()?, Token::NewLine) {
          params.push(self.operand()?);
        }
        
        Ok(Instr { kind: InstrKind::Call { dest: None, name, src: params }, line: self.cur_line })
      },

      _ => Err(self.err_to_line_end(ParseErrorKind::UnknownInstr(self.lexer.slice().to_string()))),
    }
  }

  fn block_inner(&mut self) -> ParseResult<(Vec<Instr>, Branch)> {
    let mut lines = vec![];

    loop {
      match self.peek()? {
        Token::Ret | Token::Jmp | Token::Cmp | 
        Token::Jz | Token::Jnz | Token::Je | Token::Jne |
        Token::Jl | Token::Jle | Token::Jg | Token::Jge | 
        Token::Jnl | Token::Jnle | Token::Jng | Token::Jnge => break,
        _ => {
          lines.push(self.instr()?);
          self.skip_opt_newlines();
        }
      } 
    }

    let branch = match self.token()? {
      Token::Ret => {
        let temp_opt = if matches!(self.peek()?, Token::NewLine) {
          None
        } else {
          Some(self.operand()?)
        };

        BranchKind::Ret(temp_opt)
      },

      Token::Cmp => {
        let cond = {
          let loper = self.operand()?;
          if matches!(self.peek()?, Token::Block(_)) {
            Cond::Value(loper)

          } else {
            let op = binop_code(self.token()?, self.lexer.span())?;
            let roper = self.operand()?;
            Cond::BinOp(loper, op, roper)
          }
        };

        let lblock = self.block()?;
        let rblock = self.block()?;
        BranchKind::Cond(cond, lblock, rblock)
      },

      Token::Jmp => BranchKind::Jump(self.block()?),
      Token::Jz => BranchKind::CondJump(CondJumpKind::Zero, self.block()?, self.block()?),
      Token::Jnz => BranchKind::CondJump(CondJumpKind::NotZero, self.block()?, self.block()?),
      Token::Je => BranchKind::CondJump(CondJumpKind::Equal, self.block()?, self.block()?),
      Token::Jne => BranchKind::CondJump(CondJumpKind::NotEqual, self.block()?, self.block()?),
      Token::Jl => BranchKind::CondJump(CondJumpKind::Less, self.block()?, self.block()?),
      Token::Jle => BranchKind::CondJump(CondJumpKind::LessEqual, self.block()?, self.block()?),
      Token::Jg => BranchKind::CondJump(CondJumpKind::Greater, self.block()?, self.block()?),
      Token::Jge => BranchKind::CondJump(CondJumpKind::GreaterEqual, self.block()?, self.block()?),
      Token::Jnl => BranchKind::CondJump(CondJumpKind::NotLess, self.block()?, self.block()?),
      Token::Jnle => BranchKind::CondJump(CondJumpKind::NotLessEqual, self.block()?, self.block()?),
      Token::Jng => BranchKind::CondJump(CondJumpKind::NotGreater, self.block()?, self.block()?),
      Token::Jnge => BranchKind::CondJump(CondJumpKind::NotGreaterEqual, self.block()?, self.block()?),

      _ => unreachable!(),
    };

    self.skip_newlines()?;
    Ok((lines, Branch { kind: branch, line: self.cur_line }))
  }

  fn blocks(&mut self) -> ParseResult<FxHashMap<BlockID, BasicBlock>> {
    let mut blocks = FxHashMap::default();

    loop {
      match self.peek() {
        Ok(Token::Id(_)) | Err(ParseError(ParseErrorKind::EOF, _)) => {
          return if blocks.is_empty() { Err(self.err(ParseErrorKind::FuncNeedBlock)) } else { Ok(blocks) }
        },

        Ok(Token::Block(_)) => {
          let line_start = self.cur_line;
          let id = self.block()?;
          // Parse List of Predecessors
          let mut preds = vec![];
          while !matches!(self.peek()?, Token::NewLine) {
            preds.push(self.block()?);
          }

          self.skip_newlines()?;
          let (lines, branch) = self.block_inner()?;
          blocks.insert(id, BasicBlock { id, preds, lines, branch, line_start });
        },

        Ok(tok) => {
          let tok = tok.clone();
          return Err(self.err(ParseErrorKind::NoBlock(tok)))
        },

        Err(err) => return Err(err),
      }
    }
  }
  
  fn func(&mut self) -> ParseResult<Func> {
    let line_start = self.cur_line;
    let name = self.name()?;

    // Parse List of Parameters
    let mut params = vec![];
    while !matches!(self.peek()?, Token::NewLine) {
      params.push(self.temp()?);
    }

    self.skip_newlines()?;

    // Parser Blocks (or single block)
    let blocks = if matches!(self.peek()?, Token::Temp(_) | Token::Ret) { 
      let line_start = self.cur_line;
      let mut map = FxHashMap::default();
      let (lines, branch) = self.block_inner()?;
      map.insert(BlockID(0), BasicBlock { id: BlockID(0), preds: vec![], lines, branch, line_start });
      map
    
    } else {
      self.blocks()?
    };

    Ok(Func { name, params, blocks, line_start, count: None })
  }

  fn asm(&mut self) -> ParseResult<ASM> {  
    let mut funcs = FxHashMap::default();
    self.skip_opt_newlines();

    while !matches!(self.peek(), Err(ParseError(ParseErrorKind::EOF, _))) {
      let func = self.func()?;
      funcs.insert(func.name.clone(), func);
      // self.skip_newlines();
    }

    Ok(funcs)
  }
}

// Parses the file string into an ASM
pub fn parse(file_str: &str) -> ParseResult<ASM> {
  let lexer = Token::lexer(file_str);
  let mut parser = Parser { peeked: None, lexer, cur_line: 1 };
  parser.asm()
}
