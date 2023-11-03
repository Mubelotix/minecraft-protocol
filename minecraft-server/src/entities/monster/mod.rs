use super::*;

mod base_piglin;
pub use base_piglin::*;

#[derive(Default)]
#[inheritable]
#[inherit(PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Monster {
    pub pathfinder_mob: PathfinderMob,
}
