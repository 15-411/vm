use std::fmt::{Display, Formatter, self};
use derives::DebugFromDisplay;


/// A struct with no fields.
#[derive(DebugFromDisplay)]
struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Display for Pair<T> where T: Display {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}


fn main() {
  let pair = Pair { x: 10, y: 20 };

  println!("Display Pairs: {}", pair);
  println!("Debug Pairs: {}", pair);
}
