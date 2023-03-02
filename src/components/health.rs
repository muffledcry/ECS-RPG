#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
  current: i32,
  max: i32,
}

impl Health {
  pub fn new(max: i32) -> Health {
    Health { 
      current: max, 
      max 
    }
  }

  pub fn decrease(&mut self, amount: i32) {
    self.current -= amount;
    if self.current < 0 {
      self.current = 0;
    }
  }

  pub fn increase(&mut self, amount: i32) {
    self.current += amount;
      if self.current > self.max {
        self.current = self.max;
      }
    }

  pub fn is_alive(&self) -> bool {
    self.current > 0
  }

  pub fn get_current(&self) -> i32 {
    self.current
  }

  pub fn get_max(&self) -> i32 {
    self.max
  }

  pub fn increase_max(&mut self, increment: i32) {
    self.max += increment;
  }
}
