use super::*;

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

pub trait ThrownItemProjectileDescendant: EntityDescendant {
    fn get_thrown_item_projectile(&self) -> &ThrownItemProjectile;
    fn get_thrown_item_projectile_mut(&mut self) -> &mut ThrownItemProjectile;
}

impl EntityDescendant for ThrownItemProjectile {
    fn get_entity(&self) -> &Entity { &self.entity }
    fn get_entity_mut(&mut self) -> &mut Entity { &mut self.entity }
}

impl ThrownItemProjectileDescendant for ThrownItemProjectile {
    fn get_thrown_item_projectile(&self) -> &ThrownItemProjectile { self }
    fn get_thrown_item_projectile_mut(&mut self) -> &mut ThrownItemProjectile { self }
}
