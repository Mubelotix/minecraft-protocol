use super::*;

mod villagers;
pub use villagers::*;
mod golems;
pub use golems::*;
mod ender_dragon;
pub use ender_dragon::*; 
mod slime;
pub use slime::*;
mod flying;
pub use flying::*;
mod bat;
pub use bat::*;

#[derive(Default, Clone)]
#[MinecraftEntity(
    inheritable,
    ancestors { LivingEntity, Entity },
    descendants { AmbientCreature..., PathfinderMob..., EnderDragon, Flying..., Slime },
)]
pub struct Mob {
    pub living_entity: LivingEntity,
    pub no_ai: bool,
    pub is_left_handed: bool,
    pub is_aggressive: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    inheritable,
    ancestors { Mob, LivingEntity, Entity },
    descendants { Bat },
)]
pub struct AmbientCreature {
    pub mob: Mob,
}

#[derive(Default, Clone)]
#[MinecraftEntity(
    inheritable,
    ancestors { Mob, LivingEntity, Entity },
    descendants { WaterAnimal..., AgeableMob..., Monster..., AbstractGolem... },
)]
pub struct PathfinderMob {
    pub mob: Mob,
}

#[derive(Default)]
#[MinecraftEntity(
    inheritable,
    ancestors { PathfinderMob, Mob, LivingEntity, Entity },
    descendants { Animal..., AbstractVillager... },
)]
pub struct AgeableMob {
    pub pathfinder_mob: PathfinderMob,
    pub is_baby: bool,
}
