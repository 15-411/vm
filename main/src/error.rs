use std::fmt::{Display, Formatter, Error as FmtError};
use std::ops::Range;

use crate::parser::error::ParseError;
use crate::analysis::SemError;


pub trait ErrorTrait {
  fn code(&self) -> u64;
  fn message(&self) -> &'static str;
  fn label(&self) -> Option<(String, Range<usize>)>;
  fn note(&self) -> Option<String>;
}

pub enum Error {
  ParseError(ParseError),
  SemError(SemError)
}

impl Error {
  pub const fn tag(&self) -> &'static str {
    match self {
      Self::ParseError(_) => "P",
      Self::SemError(_) => "S",
    }
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
    write!(f, "error[{}{}]: {}", self.tag(), self.code(), self.message())?;
    if let Some((label, _)) = self.label() {
      write!(f, " - {}", label)?;
    }

    Ok(())
  }
}

impl ErrorTrait for Error {
  fn code(&self) -> u64 {
    match self {
      Self::ParseError(e) => e.code(),
      Self::SemError(e) => e.code(),
    }
  }

  fn message(&self) -> &'static str {
    match self {
      Self::ParseError(e) => e.message(),
      Self::SemError(e) => e.message(),
    }
  }

  fn label(&self) -> Option<(String, Range<usize>)> {
    match self {
      Self::ParseError(e) => e.label(),
      Self::SemError(e) => e.label(),
    }
  }

  fn note(&self) -> Option<String> {
    match self {
      Self::ParseError(e) => e.note(),
      Self::SemError(e) => e.note(),
    }
  }
}
