use crate::entities::entity::Entity;
use crate::components::health;
use crate::components::Magic;

pub struct Status {
    effects: Vec<StatusEffect>,
}

impl Status {
  pub fn new() -> Status {
    Status { 
      effects: vec![] 
    }
  }

  pub fn add_effect(&mut self, effect: StatusEffect) {
    self.effects.push(effect);
  }

  pub fn remove_effect(&mut self, effect: &StatusEffect) {
    if let Some(index) = self.effects.iter().position(|e| *e == *effect) {
      self.effects.remove(index);
    }
  }

  pub fn has_effect(&self, effect: &StatusEffect) -> bool {
    self.effects.contains(effect)
  }

  pub fn apply_effects(&mut self, target: &mut Entity) {
    for effect in &self.effects {
      effect.apply(target);
    }
  }
}

#[derive(PartialEq)]
pub enum StatusEffect {
  Poison(i32),
  Paralyze(i32),
  Sick(i32),
  Mezmerized(i32),
}

impl StatusEffect {
  pub fn apply(&self, target: &mut Entity) {
    match self {
      StatusEffect::Poison(amount) => {
        if let Some(mut health) = target.get_component_mut::<Health>() {
          health.decrease(*amount);
        }
      }
      StatusEffect::Paralyze(duration) => {
        if let Some(mut actions) = target.get_component_mut::<Actions>() {
          actions.set_disabled(*duration);
        }
      }
      StatusEffect::Sick(amount) => {
        if let Some(mut magic) = target.get_component_mut::<Magic>() {
          magic.decrease(*amount);
        }
      }
      StatusEffect::Mezmerized(duration) => {
        if let Some(mut actions) = target.get_component_mut::<Actions>() {
          actions.decrease_frequency(*duration);
        }
      }
    }
  }
}
