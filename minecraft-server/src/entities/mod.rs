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
mod pathfinder_mob;
pub use pathfinder_mob::*;
mod water_animal;
pub use water_animal::*;
mod squid;
pub use squid::*;
mod interaction;
pub use interaction::*;
mod ageable_mob;
pub use ageable_mob::*;
mod animal;
pub use animal::*;
mod sniffer;
pub use sniffer::*;
mod abstract_horse;
pub use abstract_horse::*;
mod display;
pub use display::*;

pub use minecraft_server_derive::{inherit, inheritable};

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
    PathfinderMob(PathfinderMob),
    WaterAnimal(WaterAnimal),
    Squid(Squid),
    Animal(Animal),
    Sniffer(Sniffer),
    AbstractHorse(AbstractHorse),
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
            AnyEntity::PathfinderMob(pathfinder_mob) => pathfinder_mob.get_entity(),
            AnyEntity::WaterAnimal(water_animal) => water_animal.get_entity(),
            AnyEntity::Squid(squid) => squid.get_entity(),
            AnyEntity::AgeableMob(ageable_mob) => ageable_mob.get_entity(),
            AnyEntity::Animal(animal) => animal.get_entity(),
            AnyEntity::Sniffer(sniffer) => sniffer.get_entity(),
            AnyEntity::AbstractHorse(abstract_horse) => abstract_horse.get_entity(),
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
            AnyEntity::PathfinderMob(pathfinder_mob) => return Some(&pathfinder_mob.mob),
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

    pub fn as_pathfinder_mob(&self) -> Option<&PathfinderMob> {
        match self {
            AnyEntity::PathfinderMob(pathfinder_mob) => return Some(pathfinder_mob),
            _ => (),
        }
        if let Some(water_animal) = self.as_water_animal() {
            return Some(&water_animal.pathfinder_mob);
        }
        None
    }

    pub fn as_water_animal(&self) -> Option<&WaterAnimal> {
        match self {
            AnyEntity::WaterAnimal(water_animal) => Some(water_animal),
            AnyEntity::Squid(squid) => Some(&squid.water_animal),
            _ => None,
        }
    }
}
