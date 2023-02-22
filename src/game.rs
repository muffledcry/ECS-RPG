use crate::entities::entity_manager::EntityManager;
use crate::components::{name::Name, health::Health, location::Location};


pub struct Game {
    entity_manager: EntityManager,
}
impl Game {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
        }
    }

    pub fn init(&mut self) {
        let player_id = self.entity_manager.create_entity();
        let mut player = self.entity_manager.get_entity_mut(player_id).unwrap();
        let player_name = Name::new("Player");
        player.add_component(player_name);
        let player_pos = Location::new(0.0, 0.0);
        player.add_component(player_pos);


        for i in 0..10 {
            let monster_id = self.entity_manager.create_entity();
            {
                let mut monster = self.entity_manager.get_entity_mut(monster_id).unwrap();
                let monster_name = Name::new(&format!("Monster {}", i));
                let monster_health = Health::new(10);
                let monster_location = Location::new(i as f32, i as f32);
                monster.add_component(monster_name);
                monster.add_component(monster_health);
                monster.add_component(monster_location);
            }
        }
    }
}

