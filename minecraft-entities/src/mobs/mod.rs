use super::*;

mod squid;
pub use squid::*;
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

#[derive(Default)]
#[inheritable]
#[inherit(LivingEntity, Entity)]
pub struct Mob {
    pub living_entity: LivingEntity,
    pub no_ai: bool,
    pub is_left_handed: bool,
    pub is_aggressive: bool,
}

#[derive(Default)]
#[inheritable]
#[inherit(Mob, LivingEntity, Entity)]
pub struct AmbientCreature {
    pub mob: Mob,
}

#[derive(Default)]
#[inheritable]
#[inherit(Mob, LivingEntity, Entity)]
pub struct PathfinderMob {
    pub mob: Mob,
}

#[derive(Default)]
#[inheritable]
#[inherit(PathfinderMob, Mob, LivingEntity, Entity)]
pub struct AgeableMob {
    pub pathfinder_mob: PathfinderMob,
    pub is_baby: bool,
}
