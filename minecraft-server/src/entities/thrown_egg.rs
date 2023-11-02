use super::*;

pub struct ThrownEgg {
    pub thrown_item_projectile: ThrownItemProjectile,
}

impl Default for ThrownEgg {
    fn default() -> Self {
        ThrownEgg {
            thrown_item_projectile: ThrownItemProjectile {
                entity: Entity::default(),
                item: Slot {item: Some(SlotItem {
                    item_id: Item::Egg,
                    item_count: 1,
                    nbt_data: NbtTag::Null
                })},
            }
        }
    }
}

impl EntityDescendant for ThrownEgg {
    fn get_entity(&self) -> &Entity {
        &self.thrown_item_projectile.entity
    }

    fn get_entity_mut(&mut self) -> &mut Entity {
        &mut self.thrown_item_projectile.entity
    }
}

impl ThrownItemProjectileDescendant for ThrownEgg {
    fn get_thrown_item_projectile(&self) -> &ThrownItemProjectile {
        &self.thrown_item_projectile
    }

    fn get_thrown_item_projectile_mut(&mut self) -> &mut ThrownItemProjectile {
        &mut self.thrown_item_projectile
    }
}
