use super::*;

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
