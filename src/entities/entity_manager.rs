use super::entity::Entity;
use crate::components::entity_type::EntityType;
use std::collections::HashMap;
use uuid::Uuid;

pub struct EntityManager {
  entities: HashMap<Uuid, Entity>,
}

impl EntityManager {
  pub fn new() -> Self {
    Self {
      entities: HashMap::new(),
    }
  }

  pub fn create_entity(&mut self) -> Uuid {
    let entity = Entity::new(EntityType::Unknown);
    let id = entity.get_id();
    self.add_entity(entity);
    id //Use this returned ID to add components.
  }

  pub fn add_entity(&mut self, entity: Entity) {
    let id = entity.get_id();
    self.entities.insert(id, entity);
  }

  pub fn remove_entity(&mut self, id: Uuid) -> Option<Entity> {
    self.entities.remove(&id)
  }

  pub fn get_entity(&self, id: Uuid) -> Option<&Entity> {
    self.entities.get(&id)
  }

  pub fn get_entity_mut(&mut self, id: Uuid) -> Option<&mut Entity> {
    self.entities.get_mut(&id)
  }

}
