mod entity;
pub use entity::*;
mod thrown_item_projectile;
pub use thrown_item_projectile::*;
mod thrown_egg;
pub use thrown_egg::*;
mod thrown_ender_pearl;
pub use thrown_ender_pearl::*;
mod boat;
pub use boat::*;
mod chest_boat;
pub use chest_boat::*;

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
    ThrownEgg(ThrownEgg),
    ThrownEnderPearl(ThrownEnderPearl),
    // TODO some projectiles
    Boat(Boat),
    ChestBoat(ChestBoat),
}

impl AnyEntity {
    pub fn as_entity(&self) -> &Entity {
        match self {
            AnyEntity::Entity(entity) => entity,
            AnyEntity::ThrownItemProjectile(throw_item_projectile) => throw_item_projectile.get_entity(),
            AnyEntity::ThrownEgg(throw_egg) => throw_egg.get_entity(),
            AnyEntity::ThrownEnderPearl(throw_ender_pearl) => throw_ender_pearl.get_entity(),
            AnyEntity::Boat(boat) => boat.get_entity(),
            AnyEntity::ChestBoat(chest_boat) => chest_boat.get_entity(),
        }
    }

    pub fn as_thrown_item_projectile(&self) -> Option<&ThrownItemProjectile> {
        match self {
            AnyEntity::ThrownItemProjectile(throw_item_projectile) => Some(throw_item_projectile),
            AnyEntity::ThrownEgg(throw_egg) => Some(&throw_egg.thrown_item_projectile),
            AnyEntity::ThrownEnderPearl(throw_ender_pearl) => Some(&throw_ender_pearl.thrown_item_projectile),
            _ => None,
        }
    }

    pub fn as_boat(&self) -> Option<&Boat> {
        match self {
            AnyEntity::Boat(boat) => Some(boat),
            AnyEntity::ChestBoat(chest_boat) => Some(&chest_boat.boat),
            _ => None,
        }
    }
}
