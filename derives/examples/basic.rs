use std::fmt::{Display, Formatter, self};
use derives::DebugFromDisplay;


/// A struct with no fields.
#[derive(DebugFromDisplay)]
struct Pair {
  x: i32,
  y: i32,
}

impl Display for Pair {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}


fn main() {
  let pair = Pair { x: 10, y: 20 };

  println!("Display Pairs: {}", pair);
  println!("Debug Pairs: {}", pair);
}
