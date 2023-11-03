use super::*;

#[inheritable]
#[inherit(Entity)]
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

#[inherit(ThrownItemProjectile, Entity)]
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

#[inherit(ThrownItemProjectile, Entity)]
pub struct ThrownEnderPearl {
    pub thrown_item_projectile: ThrownItemProjectile,
}

impl Default for ThrownEnderPearl {
    fn default() -> Self {
        ThrownEnderPearl {
            thrown_item_projectile: ThrownItemProjectile {
                entity: Entity::default(),
                item: Slot {item: Some(SlotItem {
                    item_id: Item::EnderPearl,
                    item_count: 1,
                    nbt_data: NbtTag::Null
                })},
            }
        }
    }
}
