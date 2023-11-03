use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Monster {
    pub pathfinder_mob: PathfinderMob,
}
