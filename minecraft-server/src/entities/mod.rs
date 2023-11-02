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
mod living_entity;
pub use living_entity::*;
mod player;
pub use player::*;
mod mob;
pub use mob::*;

pub use crate::prelude::*;
pub use minecraft_protocol::{
    components::{
        entity::Pose,
        slots::{Slot, SlotItem, Hand}
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
    LivingEntity(LivingEntity),
    Player(Player),
    Mob(Mob),
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
            AnyEntity::LivingEntity(living_entity) => living_entity.get_entity(),
            AnyEntity::Player(player) => player.get_entity(),
            AnyEntity::Mob(mob) => mob.get_entity(),
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

    pub fn as_living_entity(&self) -> Option<&LivingEntity> {
        match self {
            AnyEntity::LivingEntity(living_entity) => Some(living_entity),
            AnyEntity::Player(player) => Some(&player.living_entity),
            AnyEntity::Mob(mob) => Some(&mob.living_entity),
            _ => None,
        }
    }

    pub fn as_mob(&self) -> Option<&Mob> {
        match self {
            AnyEntity::Mob(mob) => Some(mob),
            _ => None,
        }
    }
}
