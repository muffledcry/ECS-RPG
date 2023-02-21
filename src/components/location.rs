pub struct Location {
  pub x: f32,
  pub y: f32,
}

impl Location {
  pub fn new(x: f32, y: f32) -> Location {
    Location { 
      x: x, 
      y: y 
    }
  }
}
