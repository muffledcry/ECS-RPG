pub struct Attributes {
  strength: i32,
  constitution: i32,
  dexterity: i32,
  wisdom: i32,
  intelligence: i32,
  charisma: i32,
  luck: i32,
}

impl Default for Attributes {
  fn default() -> Attributes {
    Attributes {
      strength: 1,
      constitution: 1,
      dexterity: 1,
      wisdom: 1,
      intelligence: 1,
      charisma: 1,
      luck: 1,
    }
  }
}

impl Attributes {
  pub fn new(
    strength: i32, constitution: i32, dexterity: i32, wisdom: i32, intelligence: i32, charisma: i32, luck: i32) -> Attributes {
    
    Attributes {
      strength,
      constitution,
      dexterity,
      wisdom,
      intelligence,
      charisma,
      luck,
    }
  }

  pub fn get_strength(&self) -> i32 {
    self.strength
  }

  pub fn set_strength(&mut self, value: i32) {
    self.strength = value;
  }
  
  pub fn get_constitution(&self) -> i32 {
    self.constitution
  }
  
  pub fn set_constitution(&mut self, value: i32) {
    self.constitution = value;
  }
  
  pub fn get_dexterity(&self) -> i32 {
    self.dexterity
  }

  pub fn set_dexterity(&mut self, value: i32) {
    self.dexterity = value;
  }
  
  pub fn get_wisdom(&self) -> i32 {
    self.wisdom
  }

  pub fn set_wisdom(&mut self, value: i32) {
    self.wisdom = value;
  }
  
  pub fn get_intelligence(&self) -> i32 {
    self.intelligence
  }

  pub fn set_intelligence(&mut self, value: i32) {
    self.intelligence = value;
  }
  
  pub fn get_charisma(&self) -> i32 {
    self.charisma
  }

  pub fn set_charisma(&mut self, value: i32) {
    self.charisma = value;
  }
  
  pub fn get_luck(&self) -> i32 {
    self.luck
  }

  pub fn set_luck(&mut self, value: i32) {
    self.luck = value;
  }
}
