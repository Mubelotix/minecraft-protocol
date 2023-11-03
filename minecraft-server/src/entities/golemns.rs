use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(PathfinderMob, Mob, LivingEntity, Entity)]
pub struct AbstractGolem {
    pub pathfinder_mob: PathfinderMob,
}
