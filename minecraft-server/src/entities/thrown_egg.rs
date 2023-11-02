use super::*;

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
