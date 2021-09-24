mod lexer;
mod error;
mod utils;

use std::iter::Peekable;

use fxhash::FxHashMap;
use logos::{Logos, Lexer};

use crate::asm::ASM;
use crate::asm::blocks::{Func, BasicBlock, BlockID, Branch, Cond};
use crate::asm::instr::{Instr, Operand, Temp};

use lexer::Token;
use error::{Result, Error, errs, err};
use utils::{unop_code, binop_code};


#[derive(Clone)]
struct Parser<'a> {
  lexer: Peekable<Lexer<'a, Token>>,
}

impl<'a> Parser<'a> {
  // ---------------------------- HELPER FUNCTIONS ----------------------------
  /// Get the next token from the lexer
  /// Returns EOF when no more tokens
  fn token(&mut self) -> Result<Token> {
    self.lexer.next().ok_or(Error::EOF)
  }
  
  /// Look at the next token in the lexer stream.
  /// This function NEVER moves the lexer, and can be called multiple times
  /// and will return the same result.
  fn peek(&mut self) -> Result<Token> {
    // TODO: Unstable use function .cloned() instead of map
    self.lexer.peek().ok_or(Error::EOF).map(|x| x.clone())
  }

  // Skips to the next token without checking what it is
  fn skip(&mut self) -> Result<()> {
    self.token()?;
    Ok(())
  }

  fn skip_newlines(&mut self) -> Result<()> {
    self.munch(Token::NewLine)?;
    self.skip_opt_newlines();

    Ok(())
  }

  fn skip_opt_newlines(&mut self) {
    while matches!(self.peek(), Ok(Token::NewLine)) {
      self.skip().expect("Should Always Work");
    }
  }

  // Skips to the next token, verifying that this has the value we expect.
  fn munch(&mut self, tok: Token) -> Result<()> {
    let ltok = self.token()?;
    if ltok == tok {
      Ok(())
    } else {
      errs(format!("Expected {:?}, got {:?}", tok, ltok))
    }
  }

  // Expect next token to be a temp and get inner count
  fn temp(&mut self) -> Result<Temp> {
    let tok  = self.token()?;
    if let Token::Temp(val) = tok {
      Ok(Temp(val))
    } else {
      errs(format!("Expected temp, got {:?}", tok))
    }
  }

  fn block(&mut self) -> Result<BlockID> {
    let tok  = self.token()?;
    if let Token::Block(val) = tok {
      Ok(BlockID(val))
    } else {
      errs(format!("Expected block, go {:?}", tok))
    }
  }

  // Expect next token to be an ID and get inner name
  fn name(&mut self) -> Result<String> {
    let tok = self.token()?;
    if let Token::Id(name) = tok {
      Ok(name)
    } else {
      errs(format!("Expected identifier, got {:?}", tok))
    }
  }


  // ---------------------------- PARSER FUNCTIONS ----------------------------
  fn operand(&mut self) -> Result<Operand> {
    match self.token()? {
      Token::Temp(val) => Ok(Operand::Temp(Temp(val))),
      Token::Const(val) => Ok(Operand::Const(val as i32)),
      _ => err("Invalid Operand"),
    }
  }

  fn mov_binop_instr(&mut self, dest: Temp, lsrc: Operand) -> Result<Instr> {
    match self.token()? {
      Token::NewLine => Ok(Instr::Mov { dest, src: lsrc }),
      tok => {
        let op = binop_code(tok)?;
        let src2 = self.operand()?;

        self.munch(Token::NewLine)?;
        Ok(Instr::BinOp { dest, op, src1: lsrc, src2 })
      }
    }
  }

  fn instr(&mut self) -> Result<Instr> {
    match self.token()? {
      Token::Temp(val) => {
        let dest = Temp(val);
        self.munch(Token::Assign)?;

        match self.token()? {
          op @ (Token::Sub | Token::LogNot | Token::BitNot) => {
            let src = self.operand()?;
            self.munch(Token::NewLine)?;
            Ok(Instr::UnOp { dest, src, op: unop_code(op)? })
          },

          Token::Phi => {
            let mut srcs = vec![];
            loop {
              match self.token()? {
                Token::Temp(val) => { srcs.push(Operand::Temp(Temp(val))); },
                Token::Const(val) => { srcs.push(Operand::Const(val)); },
                Token::NewLine => break,
                _ => unreachable!(),
              }
            }

            Ok(Instr::Phi { dest, srcs })
          },

          Token::Call => {
            let name = self.name()?;
            let mut params = vec![];
            while self.peek()? != Token::NewLine {
              params.push(self.operand()?);
            }
            
            Ok(Instr::Call { dest: Some(dest), name, src: params })
          },

          Token::Temp(val) =>
            self.mov_binop_instr(dest, Operand::Temp(Temp(val))),
          Token::Const(val) =>
            self.mov_binop_instr(dest, Operand::Const(val)),
          _ => unreachable!(),
        }
      },

      Token::If => {
        let cond = self.operand()?;
        let block = self.block()?;
        Ok(Instr::If { cond, block })
      },

      Token::Print => {
        let value = self.operand()?;
        Ok(Instr::Print { value })
      },

      Token::Dump => {
        Ok(Instr::Dump)
      },

      Token::Call => {
        let name = self.name()?;
        let mut params = vec![];
        while self.peek()? != Token::NewLine {
          params.push(self.operand()?);
        }
        
        Ok(Instr::Call { dest: None, name, src: params })
      },

      _ => unreachable!(),
    }
  }

  fn block_inner(&mut self) -> Result<(Vec<Instr>, Branch)> {
    let mut lines = vec![];

    loop {
      match self.peek()? {
        Token::Ret | Token::Jmp | Token::Cmp => break,
        _ => {
          lines.push(self.instr()?);
          self.skip_opt_newlines();
        }
      } 
    }

    let branch = match self.token()? {
      Token::Ret => {
        let temp_opt = if self.peek()? == Token::NewLine {
          None
        } else {
          Some(self.operand()?)
        };

        Branch::Ret(temp_opt)
      },

      Token::Cmp => {
        let cond = {
          let loper = self.operand()?;
          if matches!(self.peek()?, Token::Block(_)) {
            Cond::Value(loper)

          } else {
            let op = binop_code(self.token()?)?;
            let roper = self.operand()?;
            Cond::BinOp(loper, op, roper)
          }
        };

        let lblock = self.block()?;
        let rblock = self.block()?;
        Branch::Cond(cond, lblock, rblock)
      },

      Token::Jmp => Branch::Jump(self.block()?),
      _ => unreachable!(),
    };

    self.skip_newlines()?;
    Ok((lines, branch))
  }

  fn blocks(&mut self) -> Result<FxHashMap<BlockID, BasicBlock>> {
    let mut blocks = FxHashMap::default();

    loop {
      match self.peek() {
        Ok(Token::Id(_)) | Err(Error::EOF) => {
          return if blocks.is_empty() { err("Function Needs At Least 1 Block") } else { Ok(blocks) }
        },

        Ok(Token::Block(_)) => {
          let id = self.block()?;
          // println!("Block {:?}", self.lexer.clone().into_iter().collect::<Vec<_>>());
          // Parse List of Predecessors
          let mut preds = vec![];
          while self.peek()? != Token::NewLine {
            preds.push(self.block()?);
          }

          self.skip_newlines()?;
          // println!("After Preds {:?}", self.lexer.clone().into_iter().collect::<Vec<_>>());
          let (lines, branch) = self.block_inner()?;
          blocks.insert(id, BasicBlock { id, preds, lines, branch });
        },

        _ => return err("Invalid Block Header"),
      }
    }
  }
  
  fn func(&mut self) -> Result<Func> {
    let name = self.name()?;

    // Parse List of Parameters
    let mut params = vec![];
    while self.peek()? != Token::NewLine {
      params.push(self.temp()?);
    }

    self.skip_newlines()?;

    // Parser Blocks (or single block)
    let blocks = if matches!(self.peek()?, Token::Temp(_) | Token::Ret) { 
      let mut map = FxHashMap::default();
      let (lines, branch) = self.block_inner()?;
      map.insert(BlockID(0), BasicBlock { id: BlockID(0), preds: vec![], lines, branch });
      map
    
    } else {
      self.blocks()?
    };

    Ok(Func { name, params, blocks })
  }

  fn asm(&mut self) -> Result<ASM> {  
    let mut funcs = FxHashMap::default();
    self.skip_opt_newlines();

    while self.peek() != Err(Error::EOF) {
      let func = self.func()?;
      funcs.insert(func.name.clone(), func);
      // self.skip_newlines();
    }

    Ok(funcs)
  }
}

// Parses the file string into an ASM
pub fn parse(file_str: String) -> Result<ASM> {
  let lexer = Token::lexer(file_str.as_str()).peekable();
  // println!("{:?}", lexer.clone().into_iter().collect::<Vec<_>>());
  let mut parser = Parser { lexer };
  parser.asm()
}
