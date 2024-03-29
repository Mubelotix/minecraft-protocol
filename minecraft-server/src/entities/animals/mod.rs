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
    inheritable,
    ancestors { AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
    descendants { Axolotl, Bee, Chicken, Cow, Fox, Frog, Goat, Hoglin, Ocelot, Panda, Pig, PolarBear, Rabbit, Sheep, Sniffer, Strider, Turtle, AbstractHorse..., TameableAnimal... },
)]
pub struct Animal {
    pub ageable_mob: AgeableMob,
}

#[derive(Default)]
#[MinecraftEntity(
    inheritable,
    ancestors { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
    descendants { Cat, Parrot, Wolf }
)]
pub struct TameableAnimal {
    pub animal: Animal,
    pub action_mask: u8,
    pub owner: Option<UUID>,
}
