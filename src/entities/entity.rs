use std::any::{Any, TypeId};
use std::collections::HashMap;
use uuid::Uuid;
use crate::components::entity_type::EntityType;

pub struct Entity {
  id: Uuid,
  entity_type: EntityType,
  components: HashMap<TypeId, Box<dyn Any>>,
}

impl Entity {
  pub fn new(entity_type: EntityType) -> Entity {
    Entity {
      id: Uuid::new_v4(),
      entity_type: entity_type,
      components: HashMap::new(),
    }
  }

  pub fn get_id(&self) -> Uuid {
    self.id
  }

  pub fn add_component<T: 'static>(&mut self, component: T) {
    self.components.insert(TypeId::of::<T>(), Box::new(component));
  }

  pub fn get_component<T: 'static>(&self) -> Option<&T> {
    self.components
      .get(&TypeId::of::<T>())
      .map(|component| component.downcast_ref::<T>().unwrap())
  }

  pub fn has_component<T: 'static>(&self) -> bool {
    self.components.contains_key(&TypeId::of::<T>())
  }
}
