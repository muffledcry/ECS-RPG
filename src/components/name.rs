pub struct Name {
  pub name: String,
}

impl Name {
  pub fn new(name: &str) -> Name {
    Name { 
      name: name.to_string() 
    }
  }
  
  pub fn get_name(&self) -> String {
    self.name
  }

  pub fn change_name(&mut self, new_name: &str) {
    self.name = new_name.to_string()
  } 
}
