use std::{fmt::{Display, Error, Formatter}, str::FromStr};

use derives::DebugFromDisplay;


#[derive(Copy, Clone, DebugFromDisplay, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Register {
  EAX,
  EBX,
  ECX,
  EDX,
  EDI,
  ESI,
  EBP,
  R8D,
  R9D,
  R10D,
  R11D,
  R12D,
  R13D,
  R14D,
  R15D,
}

impl Register {
  pub const ALL: [Register; 15] = [
    Register::EAX,
    Register::EBX,
    Register::ECX,
    Register::EDX,
    Register::EDI,
    Register::ESI,
    Register::EBP,
    Register::R8D,
    Register::R9D,
    Register::R10D,
    Register::R11D,
    Register::R12D,
    Register::R13D,
    Register::R14D,
    Register::R15D,
  ];
}

impl Display for Register {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    use Register::*;

    write!(f, "{}", match self {
      EAX  => "eax",
      EBX  => "ebx",
      ECX  => "ecx",
      EDX  => "edx",
      EDI  => "edi",
      ESI  => "esi",
      EBP  => "ebp",
      R8D  => "r8d",
      R9D  => "r9d",
      R10D => "r10d",
      R11D => "r11d",
      R12D => "r12d",
      R13D => "r13d",
      R14D => "r14d",
      R15D => "r15d",
      }
    )
  }
}

impl FromStr for Register {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    use Register::*;

    match s {
      "eax"  => Ok(EAX),
      "ebx"  => Ok(EBX),
      "ecx"  => Ok(ECX),
      "edx"  => Ok(EDX),
      "edi"  => Ok(EDI),
      "esi"  => Ok(ESI),
      "ebp"  => Ok(EBP),
      "r8d"  => Ok(R8D),
      "r9d"  => Ok(R9D),
      "r10d" => Ok(R10D),
      "r11d" => Ok(R11D),
      "r12d" => Ok(R12D),
      "r13d" => Ok(R13D),
      "r14d" => Ok(R14D),
      "r15d" => Ok(R15D),
      _ => Err(())
    }
  }
}
