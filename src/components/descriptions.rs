use std::collections::HashMap;
use uuid::Uuid;

pub struct Descriptions {
  texts: HashMap<String, String>
}

impl Descriptions {
  pub fn new() -> Descriptions {
    Descriptions { 
      texts: HashMap::new()
    }
  }

  pub fn add_description(&mut self, key: &str, value: &str) {
    self.texts.insert(key.to_string(), value.to_string());
  }

  pub fn get_description(&self, id: &Uuid, key: &str) -> String {
    match self.texts.get(key) {
      Some(text) => text.to_string(),
      None => "No such description found".to_string(),
    }
  }

  pub fn get_all_descriptions(&self, id: &Uuid) -> &HashMap<String, String> {
    &self.texts
  }

  pub fn delete_description(&mut self, id: &Uuid, key: &str) {
    self.texts.remove(key);
  } 
}
