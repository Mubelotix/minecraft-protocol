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
mod ambient_creature;
pub use ambient_creature::*;
mod bat;
pub use bat::*;

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
    AmbientCreature(AmbientCreature),
    Bat(Bat),
}

#[allow(clippy::single_match)]
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
            AnyEntity::AmbientCreature(ambient_creature) => ambient_creature.get_entity(),
            AnyEntity::Bat(bat) => bat.get_entity(),
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
            AnyEntity::LivingEntity(living_entity) => return Some(living_entity),
            AnyEntity::Player(player) => return Some(&player.living_entity),
            _ => (),
        };
        if let Some(mob) = self.as_mob() {
            return Some(&mob.living_entity);
        }
        None
    }

    pub fn as_mob(&self) -> Option<&Mob> {
        match self {
            AnyEntity::Mob(mob) => return Some(mob),
            _ => (),
        };
        if let Some(ambient_creature) = self.as_ambient_creature() {
            return Some(&ambient_creature.mob);
        }
        None
    }

    pub fn as_ambient_creature(&self) -> Option<&AmbientCreature> {
        match self {
            AnyEntity::AmbientCreature(ambient_creature) => Some(ambient_creature),
            AnyEntity::Bat(bat) => Some(&bat.ambient_creature),
            _ => None,
        }
    }
}
