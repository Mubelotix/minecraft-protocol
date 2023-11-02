use super::*;
use minecraft_protocol::components::slots::Slot;

pub struct ThrownItemProjectile {
    pub entity: Entity,
    pub item: Slot,
}

impl Default for ThrownItemProjectile {
    fn default() -> Self {
        ThrownItemProjectile {
            entity: Entity::default(),
            item: Slot {item: None},
        }
    }
}

impl EntityDescendant for ThrownItemProjectile {
    fn get_entity(&self) -> &Entity {
        &self.entity
    }

    fn get_entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }
}
