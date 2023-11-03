use super::*;

mod base_piglin;
pub use base_piglin::*;
mod piglin;
pub use piglin::*;
mod blaze;
pub use blaze::*;
mod creeper;
pub use creeper::*;

#[derive(Default)]
#[inheritable]
#[inherit(PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Monster {
    pub pathfinder_mob: PathfinderMob,
}
