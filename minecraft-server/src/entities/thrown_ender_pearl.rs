use super::*;

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

impl EntityDescendant for ThrownEnderPearl {
    fn get_entity(&self) -> &Entity { &self.thrown_item_projectile.entity }
    fn get_entity_mut(&mut self) -> &mut Entity { &mut self.thrown_item_projectile.entity }
}

impl ThrownItemProjectileDescendant for ThrownEnderPearl {
    fn get_thrown_item_projectile(&self) -> &ThrownItemProjectile { &self.thrown_item_projectile }
    fn get_thrown_item_projectile_mut(&mut self) -> &mut ThrownItemProjectile { &mut self.thrown_item_projectile }
}
