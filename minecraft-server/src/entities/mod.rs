mod entity;
pub use entity::*;
mod thrown_item_projectile;
pub use thrown_item_projectile::*;
mod thrown_egg;
pub use thrown_egg::*;

pub use minecraft_protocol::{
    components::{
        entity::Pose,
        slots::{Slot, SlotItem}
    },
    ids::items::Item,
    nbt::NbtTag
};

pub enum AnyEntity {
    Entity(Entity),
    ThrownItemProjectile(ThrownItemProjectile),
}

impl AnyEntity {
    pub fn as_entity(&self) -> &Entity {
        match self {
            AnyEntity::Entity(entity) => entity,
            AnyEntity::ThrownItemProjectile(throw_item_projectile) => throw_item_projectile.get_entity(),
        }
    }

    pub fn as_thrown_item_projectile(&self) -> Option<&ThrownItemProjectile> {
        match self {
            AnyEntity::ThrownItemProjectile(throw_item_projectile) => Some(throw_item_projectile),
            _ => None,
        }
    }
}
