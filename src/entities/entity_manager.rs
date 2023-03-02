use super::entity::Entity;
use dashmap::DashMap;
use uuid::Uuid;
use crate::utils::ConnectionInfo;
use crate::components::name::Name;
use crate::components::location::Location;
use crate::components::health::Health;

#[derive(Debug)]
pub struct EntityManager {
  pub entities: DashMap<Uuid, Entity>,
  pub names: DashMap<Uuid, Name>,
  pub locations: DashMap<Uuid, Location>,
  pub healths: DashMap<Uuid, Health>,
}

impl EntityManager {
  pub fn new() -> Self {
    Self { 
      entities: DashMap::new(),
      names: DashMap::new(),
      locations: DashMap::new(),
      healths: DashMap::new(),
    }
  }

  pub fn create_entity(&mut self) -> Uuid {
    let conn_info = ConnectionInfo::default();
    let dummy_id = Uuid::new_v4();
    let entity = Entity::new(dummy_id, conn_info); // replace 1 with the relevant entity type
    let id = entity.get_id();
    self.add_entity(entity);
    id //Use this returned ID to add components.
  }

  pub fn add_entity(&mut self, entity: Entity) {
    let id = entity.get_id();
    self.entities.insert(id, entity);
  }

  pub fn remove_entity(&mut self, id: Uuid) -> Option<Entity> {
    self.entities.remove(&id).map(|(_, v)| v)
  }

  pub fn get_entity(&self, id: Uuid) -> Option<Entity> {
    self.entities.get(&id).map(|r| (*r.value()).clone())
  }

  pub fn get_entity_mut(&self, id: Uuid) -> Option<Entity> {
    self.entities.get_mut(&id).map(|r| (*r.value()).clone())
  }

}

