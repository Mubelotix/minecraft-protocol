use super::*;

mod sniffer;
pub use sniffer::*;
mod horses;
pub use horses::*;
mod axolotl;
pub use axolotl::*;
mod bee;
pub use bee::*;
mod fox;
pub use fox::*;
mod frog;
pub use frog::*;
mod ocelot;
pub use ocelot::*;
mod panda;
pub use panda::*;
mod pig;
pub use pig::*;
mod rabbit;
pub use rabbit::*;
mod turtle;
pub use turtle::*;
mod polar_bear;
pub use polar_bear::*;
mod chicken;
pub use chicken::*;
mod cow;
pub use cow::*;
mod hoglin;
pub use hoglin::*;
mod sheep;
pub use sheep::*;
mod strider;
pub use strider::*;
mod cat;
pub use cat::*;
mod wolf;
pub use wolf::*;
mod parrot;
pub use parrot::*;
mod goat;
pub use goat::*;
mod water_animal;
pub use water_animal::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, ancestors { AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Animal {
    pub ageable_mob: AgeableMob,
}

impl TryAsEntityRef<Animal> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&Animal> {
        match self {
            AnyEntity::Animal(animal) => return Some(&animal),
            _ => (),
        }
        if let Some(tameable_animal) = <Self as TryAsEntityRef<TameableAnimal>>::try_as_entity_ref(self) {
            return Some(&tameable_animal.animal)
        }
        None
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut Animal> {
        match self {
            AnyEntity::Animal(animal) => return Some(animal),
            _ => (),
        }
        if let Some(tameable_animal) = <Self as TryAsEntityRef<TameableAnimal>>::try_as_entity_mut(self) {
            return Some(&mut tameable_animal.animal)
        }
        None
    }
}

#[derive(Default)]
#[MinecraftEntity(
    inheritable, ancestors { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct TameableAnimal {
    pub animal: Animal,
    pub action_mask: u8,
    pub owner: Option<UUID>,
}

impl TryAsEntityRef<TameableAnimal> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&TameableAnimal> {
        match self {
            AnyEntity::TameableAnimal(tameable_animal) => return Some(&tameable_animal),
            _ => (),
        }
        None
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut TameableAnimal> {
        match self {
            AnyEntity::TameableAnimal(tameable_animal) => return Some(tameable_animal),
            _ => (),
        }
        None
    }
}
